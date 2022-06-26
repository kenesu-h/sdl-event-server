use serde::{Serialize, Deserialize};
use sdl2::controller::{Axis, Button};

#[derive(Serialize, Deserialize)]
pub enum SdlAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight
}

impl SdlAxis {
    pub fn is_trigger(&self) -> bool {
        match self {
            Self::TriggerLeft | Self::TriggerRight => true,
            _ => false
        }
    }

    pub fn from_raw_sdl(axis: Axis) -> Self {
        match axis {
            Axis::LeftX => Self::LeftX,
            Axis::LeftY => Self::LeftY,
            Axis::RightX => Self::RightX,
            Axis::RightY => Self::RightY,
            Axis::TriggerLeft => Self::TriggerLeft,
            Axis::TriggerRight => Self::TriggerRight
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SdlButton {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Misc1,
    Paddle1,
    Paddle2,
    Paddle3,
    Paddle4,
    Touchpad
}

impl SdlButton {
    pub fn from_raw_sdl(button: Button, controller_name: &str) -> Self {
        match button {
            Button::A => match controller_name {
                "Nintendo Switch Pro Controller" => Self::B,
                _ => Self::A
            },
            Button::B => match controller_name {
                "Nintendo Switch Pro Controller" => Self::A,
                _ => Self::B
            },
            Button::X => match controller_name {
                "Nintendo Switch Pro Controller" => Self::Y,
                _ => Self::X
            },
            Button::Y => match controller_name {
                "Nintendo Switch Pro Controller" => Self::X,
                _ => Self::Y
            },
            Button::Back => Self::Back,
            Button::Guide => Self::Guide,
            Button::Start => Self::Start,
            Button::LeftStick => Self::LeftStick,
            Button::RightStick => Self::RightStick,
            Button::LeftShoulder => Self::LeftShoulder,
            Button::RightShoulder => Self::RightShoulder,
            Button::DPadUp => Self::DPadUp,
            Button::DPadDown => Self::DPadDown,
            Button::DPadLeft => Self::DPadLeft,
            Button::DPadRight => Self::DPadRight,
            Button::Misc1 => Self::Misc1,
            Button::Paddle1 => Self::Paddle1,
            Button::Paddle2 => Self::Paddle2,
            Button::Paddle3 => Self::Paddle3,
            Button::Paddle4 => Self::Paddle4,
            Button::Touchpad => Self::Touchpad
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SdlEvent {
    AxisMotion(u32, u32, SdlAxis, i16),
    ButtonPress(u32, u32, SdlButton, bool)
}
