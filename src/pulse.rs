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
}
impl Default for PulseSettings {
    fn default() -> Self {
        Self {
            mute_device: PulseMuteDevice::All,
        }
    }
}

pub trait Mute {
    fn is_muted(&mut self) -> bool;
    fn set_muted(&mut self, muted: bool) -> ();
}

pub struct PulseControl {
    handler: SourceController,
}

impl PulseControl {
    pub fn new() -> Self {
        let handler = SourceController::create().expect("Failed to get handler");
        Self { handler }
    }
}

impl Mute for PulseControl {
    fn is_muted(&mut self) -> bool {
        let devices = &self
            .handler
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
        let devices = &self
            .handler
            .list_devices()
            .expect("Could not get list of recording devices");
        for dev in devices.clone() {
            &self.handler.set_device_mute_by_index(dev.index, muted);
        }
    }
}
