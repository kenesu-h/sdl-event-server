use crate::sdl_event::*;

use sdl2::{
    Sdl,
    GameControllerSubsystem,
    JoystickSubsystem,
    EventPump,
    event::Event,
    controller::GameController,
    VideoSubsystem
};
use std::collections::HashMap;

pub struct SdlReader {
    game_controllers: HashMap<u32, GameController>,

    game_controller: GameControllerSubsystem,
    event_pump: EventPump,
    _joystick: JoystickSubsystem,
    _video: VideoSubsystem
}

impl SdlReader {
    pub fn new() -> SdlReader {
        // Force SDL to accept joystick inputs when in the background.
        sdl2::hint::set("SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS", "1");

        let sdl_context: Sdl = sdl2::init().unwrap();

        let game_controller: GameControllerSubsystem = sdl_context.game_controller().unwrap();
        let joystick: JoystickSubsystem = sdl_context.joystick().unwrap();
        let event_pump: EventPump = sdl_context.event_pump().unwrap();
        let video: VideoSubsystem = sdl_context.video().unwrap();

        return SdlReader {
            game_controllers: HashMap::new(),

            game_controller: game_controller,
            event_pump: event_pump,
            _joystick: joystick,
            _video: video
        }
    }

    pub fn poll_events(&mut self, events: &mut Vec<SdlEvent>) -> () {
        loop {
            match self.event_pump.poll_event() {
                None => break,
                Some(event) => match event {
                    // Store and remove game controller instances to receive events from them.
                    Event::ControllerDeviceAdded { which, .. } => {
                        let instance: GameController = self.game_controller.open(which).unwrap();
                        self.game_controllers.insert(instance.instance_id(), instance);
                    },
                    Event::ControllerDeviceRemoved { which, .. } => {
                        self.game_controllers.remove(&which);
                    },

                    // Push these events.
                    Event::ControllerAxisMotion { timestamp, which, axis, value } => {
                        events.push(SdlEvent::AxisMotion(timestamp, which, to_sdl_axis(axis), value));
                    },
                    Event::ControllerButtonDown { timestamp, which, button } => {
                        events.push(SdlEvent::ButtonPress(timestamp, which, to_sdl_button(button), true));
                    },
                    Event::ControllerButtonUp { timestamp, which, button } => {
                        events.push(SdlEvent::ButtonPress(timestamp, which, to_sdl_button(button), false));
                    },

                    // Ignore all other event types.
                    _ => ()
                }
            }
        }
    }
}
