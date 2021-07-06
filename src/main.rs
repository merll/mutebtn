mod muteme;
mod pulse;

use clap::clap_app;
use crossbeam_channel::{unbounded, RecvError, RecvTimeoutError};
use hidapi::{HidDevice, HidError};
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use std::{thread, time::Duration};

use crate::muteme::{
    get_color_by_name, get_color_value, get_operation_mode_by_name, ControlMessage, DeviceEvent,
    ExecMessage, IntMessage, OperationMode,
};
use crate::pulse::{AudioMessage, Mute, PulseControl};

fn main() -> Result<(), HidError> {
    let matches = clap_app!(mutebtn =>
        (version: "0.1.0")
        (author: "Matthias Erll <matthias@erll.de>")
        (about: "Connects the MuteMe Button")
        (@arg muted_color: --("muted-color") +takes_value
         default_value[red] possible_value[red green blue yellow cyan purple white nocolor]
         "Sets the color when muted")
        (@arg unmuted_color: --("unmuted-color") +takes_value
         default_value[green] possible_value[red green blue yellow cyan purple white nocolor]
         "Sets the color when not muted")
        (@arg operation_mode: -m --mode +takes_value
         default_value[toggle] possible_value[toggle pushtotalk]
         "Sets the operation mode")
    )
    .get_matches();

    let (ctrl_sender, ctrl_receiver) = unbounded();
    let (int_sender, int_receiver) = unbounded();
    let (exec_sender, exec_receiver) = unbounded();
    let (audio_sender, audio_receiver) = unbounded();

    let audio_ctrl_sender = ctrl_sender.clone();
    let audio_thread = thread::spawn(move || -> () {
        let mut terminated = false;
        let mut pulse_control = PulseControl::new();
        while !terminated {
            let res = audio_receiver.recv();
            match res {
                Ok(AudioMessage::GetMuteStatus) => {
                    let is_muted = pulse_control.is_muted();
                    audio_ctrl_sender
                        .send(ControlMessage::PublishMuteStatus(is_muted))
                        .unwrap_or(());
                }
                Ok(AudioMessage::SetMuteStatus(new_state)) => {
                    pulse_control.set_muted(new_state);
                }
                Ok(AudioMessage::Terminate) => terminated = true,
                Err(RecvError) => terminated = true,
            }
        }
        println!("Shutting down audio thread");
    });

    let ctrl_exec_sender = exec_sender.clone();
    let ctrl_audio_sender = audio_sender.clone();
    let ctrl_self_sender = ctrl_sender.clone();
    let ctrl_thread = thread::spawn(move || -> () {
        let mut terminated = false;
        let mut muted_color = get_color_by_name(matches.value_of("muted_color").unwrap_or("red"));
        let mut unmuted_color =
            get_color_by_name(matches.value_of("unmuted_color").unwrap_or("green"));
        let mut is_muted = false;
        let mut op_mode =
            get_operation_mode_by_name(matches.value_of("operation_mode").unwrap_or("toggle"));
        let mut transition = false;
        thread::sleep(Duration::from_millis(100));

        while !terminated {
            let res = ctrl_receiver.recv_timeout(Duration::from_secs(5));
            match res {
                Ok(ControlMessage::PublishMuteStatus(state)) => {
                    if state != is_muted {
                        is_muted = state;
                        transition = false;
                    }
                }
                Ok(ControlMessage::SetColor(mute_state, color)) => {
                    if mute_state {
                        muted_color = color;
                    } else {
                        unmuted_color = color;
                    }
                    transition = false;
                }
                Ok(ControlMessage::SetMode(new_mode)) => {
                    op_mode = new_mode;
                    is_muted = true;
                    transition = false;
                }
                Ok(ControlMessage::Event(event)) => {
                    let new_state;
                    match event {
                        DeviceEvent::Touch => {
                            println!("Touch event");
                            match op_mode {
                                OperationMode::PushToTalk => new_state = false,
                                OperationMode::Toggle => new_state = is_muted,
                            }
                        }
                        DeviceEvent::Release => {
                            println!("Release event");
                            match op_mode {
                                OperationMode::PushToTalk => new_state = true,
                                OperationMode::Toggle => new_state = !is_muted,
                            }
                        }
                    };
                    if is_muted != new_state {
                        is_muted = new_state;
                        transition = false;
                    }
                }
                Ok(ControlMessage::Continue) => {}
                Ok(ControlMessage::Terminate) => terminated = true,
                Err(RecvTimeoutError::Timeout) => {
                    println!("Sending keepalive");
                    transition = false;
                }
                Err(RecvTimeoutError::Disconnected) => terminated = true,
            }

            let current_color = if is_muted {
                &muted_color
            } else {
                &unmuted_color
            };
            let effect: u8;
            if transition {
                effect = 0x40;
                transition = false;
                ctrl_audio_sender
                    .send(AudioMessage::SetMuteStatus(is_muted))
                    .unwrap_or(());
            } else {
                effect = 0x00;
                let sub_thread_sender = ctrl_self_sender.clone();
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(100));
                    sub_thread_sender
                        .send(ControlMessage::Continue)
                        .unwrap_or(());
                });
                transition = true;
            }
            let color_value = get_color_value(&current_color) + effect;
            ctrl_exec_sender
                .send(ExecMessage::SetReport(color_value))
                .unwrap_or(());
        }
    });
    let int_exec_sender = exec_sender.clone();
    let int_thread = thread::spawn(move || {
        let mut terminated = false;
        while !terminated {
            int_exec_sender
                .send(ExecMessage::ReadInterrupt)
                .unwrap_or(());
            let res = int_receiver.recv_timeout(Duration::from_millis(200));
            match res {
                Ok(IntMessage::Terminate) => terminated = true,
                Err(RecvTimeoutError::Disconnected) => terminated = true,
                Err(RecvTimeoutError::Timeout) => continue,
            }
        }
    });
    let exec_ctrl_sender = ctrl_sender.clone();
    let exec_thread = thread::spawn(move || {
        let api = hidapi::HidApi::new().unwrap();
        let device = api
            .open(muteme::DEVICE_VID, muteme::DEVICE_PID)
            .expect("Failed to open USB device");
        device
            .set_blocking_mode(false)
            .expect("Failed to set device to non-blocking mode");

        let mut terminated = false;
        let mut state = 0;

        while !terminated {
            loop {
                let data = read_interrupt(&device);
                match data {
                    Some(new_state @ 1..=2) if state != new_state => {
                        state = new_state;
                        if state == 1 {
                            exec_ctrl_sender
                                .send(ControlMessage::Event(DeviceEvent::Touch))
                                .unwrap_or(());
                        } else {
                            exec_ctrl_sender
                                .send(ControlMessage::Event(DeviceEvent::Release))
                                .unwrap_or(());
                        }
                    }
                    Some(_) => continue,
                    None => break,
                }
            }

            let res = exec_receiver.recv();
            match res {
                Ok(ExecMessage::SetReport(value)) => {
                    let data = [0x00, value];
                    let res = device.write(&data);
                    match res {
                        Ok(i) => println!("Wrote {} bytes", i),
                        Err(err) => println!("{}", err),
                    };
                }
                Ok(ExecMessage::ReadInterrupt) => continue,
                Ok(ExecMessage::Terminate) => terminated = true,
                Err(RecvError) => terminated = true,
            }
        }
    });

    let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
    let handle = signals.handle();
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            int_sender.send(IntMessage::Terminate).unwrap_or(());
            ctrl_sender.send(ControlMessage::Terminate).unwrap_or(());
            exec_sender.send(ExecMessage::Terminate).unwrap_or(());
            audio_sender.send(AudioMessage::Terminate).unwrap_or(());
        }
    });

    int_thread.join().unwrap();
    ctrl_thread.join().unwrap();
    exec_thread.join().unwrap();
    audio_thread.join().unwrap();
    handle.close();

    Ok(())
}

fn read_interrupt(device: &HidDevice) -> Option<u8> {
    let mut buf = [0u8; 8];
    let res = device.read(&mut buf);
    match res {
        Ok(_i @ 0) => None,
        Ok(_) => Some(buf[3]),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}
