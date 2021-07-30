use pulsectl::controllers::{DeviceControl, SourceController};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AudioMessage {
    GetMuteStatus,
    SetMuteStatus(bool),
    Terminate,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PulseMuteDevice {
    All,
    Default,
    Selected(String),
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PulseSettings {
    pub mute_device: PulseMuteDevice,
    pub unmute_device: Option<PulseMuteDevice>,
}
impl Default for PulseSettings {
    fn default() -> Self {
        Self {
            mute_device: PulseMuteDevice::All,
            unmute_device: Some(PulseMuteDevice::All),
        }
    }
}

pub trait Mute {
    fn is_muted(&mut self) -> bool;
    fn set_muted(&mut self, muted: bool) -> ();
}
pub struct PulseControl {
    handler: SourceController,
    settings: PulseSettings,
}

impl PulseControl {
    pub fn new(settings: PulseSettings) -> Self {
        let handler = SourceController::create().expect("Failed to get handler");
        Self { handler, settings }
    }
}
impl Mute for PulseControl {
    fn is_muted(&mut self) -> bool {
        let device = match &self.settings.unmute_device {
            Some(dev) => dev,
            None => &self.settings.mute_device,
        };
        match device {
            PulseMuteDevice::All => {
                let devices_res = &self.handler.list_devices();
                match devices_res {
                    Ok(devices) => {
                        for dev in devices {
                            if !dev.mute {
                                return false;
                            }
                        }
                    },
                    Err(_) => {
                        println!("Could not get list of recording devices");
                        return false;
                    },
                }
                true
            },
            PulseMuteDevice::Default => match self.handler.get_server_info() {
                Ok(server_info) => match server_info.default_source_name {
                    Some(device_name) => {
                        return match &self.handler.get_device_by_name(&device_name) {
                            Ok(dev) => dev.mute,
                            Err(_) => {
                                println!("Failed to find device with default source name");
                                false
                            },
                        };
                    },
                    None => {
                        println!("No default device selected");
                        false
                    },
                },
                Err(_) => {
                    println!("Failed to get server info");
                    false
                },
            },
            PulseMuteDevice::Selected(selected_device_name) => {
                return match &self.handler.get_device_by_name(&selected_device_name) {
                    Ok(dev) => dev.mute,
                    Err(_) => {
                        println!("Failed to find device with selected source name");
                        false
                    },
                };
            },
        }
    }

    fn set_muted(&mut self, muted: bool) -> () {
        let device;
        if muted {
            device = &self.settings.mute_device;
        } else {
            device = match &self.settings.unmute_device {
                Some(dev) => dev,
                None => &self.settings.mute_device,
            };
        }
        match device {
            PulseMuteDevice::All => {
                let devices_res = &self.handler.list_devices();
                match devices_res {
                    Ok(devices) => {
                        for dev in devices {
                            &self.handler.set_device_mute_by_index(dev.index, muted);
                        }
                    },
                    Err(_) => {
                        println!("Could not get list of recording devices")
                    },
                }
            },
            PulseMuteDevice::Default => match self.handler.get_server_info() {
                Ok(server_info) => match server_info.default_source_name {
                    Some(device_name) => {
                        &self.handler.set_device_mute_by_name(&device_name, muted);
                    },
                    None => {
                        println!("No default device selected");
                    },
                },
                Err(_) => {
                    println!("Failed to get server info");
                },
            },
            PulseMuteDevice::Selected(selected_device_name) => {
                &self
                    .handler
                    .set_device_mute_by_name(&selected_device_name, muted);
            },
        }
    }
}
