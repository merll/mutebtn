use serde::{Deserialize, Serialize};

pub const DEVICE_VID: u16 = 0x20a0;
pub const DEVICE_PID: u16 = 0x42da;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Purple,
    White,
    NoColor,
}
impl Color {
    pub fn get_byte_value(&self) -> u8 {
        match self {
            Self::Red => 0x01,
            Self::Green => 0x02,
            Self::Blue => 0x04,
            Self::Yellow => 0x03,
            Self::Cyan => 0x06,
            Self::Purple => 0x05,
            Self::White => 0x07,
            Self::NoColor => 0x00,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationMode {
    Toggle,
    PushToTalk,
}
#[derive(Debug)]
pub enum DeviceEvent {
    Touch,
    Release,
}
pub enum ControlMessage {
    PublishMuteStatus(bool),
    SetColor(bool, Color),
    SetMode(OperationMode),
    Continue,
    Event(DeviceEvent),
    Terminate,
}
pub enum IntMessage {
    Terminate,
}
pub enum ExecMessage {
    SetReport(u8),
    ReadInterrupt,
    Terminate,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MuteMeSettings {
    pub muted_color: Color,
    pub unmuted_color: Color,
    pub operation_mode: OperationMode,
}
impl Default for MuteMeSettings {
    fn default() -> Self {
        Self {
            muted_color: Color::Red,
            unmuted_color: Color::Green,
            operation_mode: OperationMode::Toggle,
        }
    }
}
