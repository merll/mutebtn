pub const DEVICE_VID: u16 = 0x20a0;
pub const DEVICE_PID: u16 = 0x42da;

#[derive(Debug)]
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
#[derive(Debug)]
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
pub enum ExecMessage {
    SetReport(u8),
    ReadInterrupt,
    Terminate,
}

pub fn get_color_by_name(name: &str) -> Color {
    match name {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "purple" => Color::Purple,
        "white" => Color::White,
        _ => Color::NoColor,
    }
}
pub fn get_operation_mode_by_name(name: &str) -> OperationMode {
    match name {
        "toggle" => OperationMode::Toggle,
        "pushtotalk" => OperationMode::PushToTalk,
        _ => OperationMode::Toggle,
    }
}
pub fn get_color_value(color: &Color) -> u8 {
    match color {
        Color::Red => 0x01,
        Color::Green => 0x02,
        Color::Blue => 0x04,
        Color::Yellow => 0x03,
        Color::Cyan => 0x06,
        Color::Purple => 0x05,
        Color::White => 0x07,
        Color::NoColor => 0x00,
    }
}
