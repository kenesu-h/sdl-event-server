use std::collections::HashMap;

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
                    Event::ControllerDeviceAdded { timestamp, which } => {
                        let instance: GameController = self.game_controller
                            .open(which)
                            .unwrap();
                        self.game_controllers
                            .insert(instance.instance_id(), instance);
                        events.push(
                            SdlEvent::ControllerAdded { timestamp, which }
                        );
                    },
                    Event::ControllerDeviceRemoved { timestamp, which } => {
                        self.game_controllers.remove(&which);
                        events.push(
                            SdlEvent::ControllerRemoved { timestamp, which }
                        );
                    },
                    Event::ControllerAxisMotion { timestamp, which, axis, value }
                        => {
                            events.push(
                                SdlEvent::AxisMotion {
                                    timestamp,
                                    which,
                                    axis: SdlAxis::from_raw_sdl(axis),
                                    value
                                }
                            );
                        },
                    Event::ControllerButtonDown { timestamp, which, button } 
                        => {
                            events.push(
                                SdlEvent::ButtonPress {
                                    timestamp,
                                    which,
                                    button: SdlButton::from_raw_sdl(
                                        button,
                                        self.game_controllers
                                            .get(&which)
                                            .expect("Failed to get game controller.")
                                            .name()
                                            .as_str()
                                    ),
                                    pressed: true
                                }
                            );
                        },
                    Event::ControllerButtonUp { timestamp, which, button } => {
                        events.push(
                            SdlEvent::ButtonPress {
                                timestamp,
                                which,
                                button: SdlButton::from_raw_sdl(
                                    button,
                                    self.game_controllers
                                        .get(&which)
                                        .expect("Failed to get game controller.")
                                        .name()
                                        .as_str()
                                ),
                                pressed: false
                            }
                        );
                    },

                    // Ignore all other event types.
                    _ => ()
                }
            }
        }
    }
}
