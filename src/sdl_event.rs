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

pub fn to_sdl_axis(axis: Axis) -> SdlAxis {
    match axis {
        Axis::LeftX => SdlAxis::LeftX,
        Axis::LeftY => SdlAxis::LeftY,
        Axis::RightX => SdlAxis::RightX,
        Axis::RightY => SdlAxis::RightY,
        Axis::TriggerLeft => SdlAxis::TriggerLeft,
        Axis::TriggerRight => SdlAxis::TriggerRight
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

pub fn to_sdl_button(button: Button) -> SdlButton {
    match button {
        Button::A => SdlButton::A,
        Button::B => SdlButton::B,
        Button::X => SdlButton::X,
        Button::Y => SdlButton::Y,
        Button::Back => SdlButton::Back,
        Button::Guide => SdlButton::Guide,
        Button::Start => SdlButton::Start,
        Button::LeftStick => SdlButton::LeftStick,
        Button::RightStick => SdlButton::RightStick,
        Button::LeftShoulder => SdlButton::LeftShoulder,
        Button::RightShoulder => SdlButton::RightShoulder,
        Button::DPadUp => SdlButton::DPadUp,
        Button::DPadDown => SdlButton::DPadDown,
        Button::DPadLeft => SdlButton::DPadLeft,
        Button::DPadRight => SdlButton::DPadRight,
        Button::Misc1 => SdlButton::Misc1,
        Button::Paddle1 => SdlButton::Paddle1,
        Button::Paddle2 => SdlButton::Paddle2,
        Button::Paddle3 => SdlButton::Paddle3,
        Button::Paddle4 => SdlButton::Paddle4,
        Button::Touchpad => SdlButton::Touchpad
    }
}

#[derive(Serialize, Deserialize)]
pub enum SdlEvent {
    AxisMotion(u32, u32, SdlAxis, i16),
    ButtonPress(u32, u32, SdlButton, bool)
}
