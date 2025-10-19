//! Utilities to handle events

use sdl3::{
    EventPump,
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
    sys::scancode::SDL_Scancode,
};

use crate::coords::ScreenPoint;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyStatus {
    pub down: bool,
    pub since: u64,
    pub mods: Mod,
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self {
            down: false,
            since: 0,
            mods: Mod::empty(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MouseBtnStatus {
    pub down: bool,
    pub pos: ScreenPoint,
    pub since: u64,
}

pub struct Events {
    pump: EventPump,
    pub mouse_pos: ScreenPoint,
    quit_timestamp: u64,
    mouse_btns: [MouseBtnStatus; 8],
    keys: [KeyStatus; SDL_Scancode::COUNT.0 as usize],
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump,
            quit_timestamp: 0,
            mouse_pos: ScreenPoint::default(),
            mouse_btns: [MouseBtnStatus::default(); 8],
            keys: [KeyStatus::default(); SDL_Scancode::COUNT.0 as usize],
        }
    }

    /// Rescan the event pump for the newest events
    pub fn scan(&mut self) {
        let now = sdl3::timer::ticks();

        let mouse_state = self.pump.mouse_state();
        self.mouse_pos = ScreenPoint::new(mouse_state.x().into(), mouse_state.y().into());

        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { timestamp } => {
                    self.quit_timestamp = timestamp;
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let idx = mouse_btn as usize;
                    self.mouse_btns[idx].down = false;
                    self.mouse_btns[idx].since = now;
                    self.mouse_btns[idx].pos = ScreenPoint::new(x.into(), y.into());
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let idx = mouse_btn as usize;
                    self.mouse_btns[idx].down = true;
                    self.mouse_btns[idx].since = now;
                    self.mouse_btns[idx].pos = ScreenPoint::new(x.into(), y.into());
                }
                Event::KeyUp {
                    scancode, keymod, ..
                } => {
                    let Some(key) = scancode else {
                        sdl3::log::log_warn(sdl3::log::Category::Input, "received unknown key");
                        continue;
                    };

                    let idx = key as usize;
                    self.keys[idx].down = false;
                    self.keys[idx].since = now;
                    self.keys[idx].mods = keymod;
                }
                Event::KeyDown {
                    scancode, keymod, ..
                } => {
                    let Some(key) = scancode else {
                        sdl3::log::log_warn(sdl3::log::Category::Input, "received unknown key");
                        continue;
                    };

                    let idx = key as usize;
                    self.keys[idx].down = true;
                    self.keys[idx].since = now;
                    self.keys[idx].mods = keymod;
                }
                _ => continue,
            }
        }
    }

    pub fn mouse_btn(&self, btn: MouseButton) -> &MouseBtnStatus {
        &self.mouse_btns[btn as usize]
    }

    pub fn key(&self, key: Keycode) -> &KeyStatus {
        &self.keys[key as usize]
    }

    pub fn quit(&self) -> bool {
        self.quit_timestamp != 0
    }
}
