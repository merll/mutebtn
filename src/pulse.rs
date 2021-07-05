
use pulsectl::controllers::{DeviceControl, SourceController};

#[derive(Debug)]
pub enum AudioMessage {
    GetMuteStatus,
    SetMuteStatus(bool),
    Terminate,
}

pub trait Mute {
    fn is_muted(&mut self) -> bool;
    fn set_muted(&mut self, muted: bool) -> ();
}

pub struct PulseControl {
    handler: SourceController,
}

impl PulseControl {
    pub fn new() -> PulseControl {
        let handler = SourceController::create().expect("Failed to get handler");
        PulseControl { handler }
    }
}

impl Mute for PulseControl {
    fn is_muted(&mut self) -> bool {
        let devices = &self.handler
            .list_devices()
            .expect("Could not get list of recording devices");
        for dev in devices.clone() {
            if !dev.mute {
                return false;
            }
        }
        true
    }
        
    fn set_muted(&mut self, muted: bool) -> () {
        let devices = &self.handler
            .list_devices()
            .expect("Could not get list of recording devices");
        for dev in devices.clone() {
            &self.handler.set_device_mute_by_index(dev.index, muted);
        }
    }
} 
