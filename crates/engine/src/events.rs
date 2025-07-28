//! Utilities to handle events

use sdl3::{
    EventPump,
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
};

use crate::coords::ScreenPoint;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Key {
    pub key: Keycode,
    pub key_mod: Mod,
}

impl Default for Key {
    fn default() -> Self {
        Self {
            key: Keycode::Unknown,
            key_mod: Mod::empty(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mouse {
    pub btn: MouseButton,
    pub pos: ScreenPoint,
    pub clicks: u8,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            btn: MouseButton::Unknown,
            pos: ScreenPoint::default(),
            clicks: 0,
        }
    }
}

pub struct Events {
    pump: EventPump,
    mouse_up: [Mouse; 8],
    mouse_down: [Mouse; 8],
    key_up: [Key; 64],
    key_down: [Key; 64],
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump,
            mouse_up: [Mouse::default(); 8],
            mouse_down: [Mouse::default(); 8],
            key_down: [Key::default(); 64],
            key_up: [Key::default(); 64],
        }
    }

    // Flush all current events
    pub fn clear(&mut self) {
        self.mouse_up[0] = Mouse::default();
        self.mouse_down[0] = Mouse::default();
        self.key_up[0] = Key::default();
        self.key_down[0] = Key::default();
    }

    /// Rescan the event pump for the newest events
    pub fn scan(&mut self) {
        let mut mouse_up = self
            .mouse_up
            .iter()
            .position(|k| k == &Mouse::default())
            .unwrap_or(self.mouse_up.len());
        let mut mouse_down = self
            .mouse_down
            .iter()
            .position(|k| k == &Mouse::default())
            .unwrap_or(self.mouse_down.len());
        let mut key_up = self
            .key_up
            .iter()
            .position(|k| k == &Key::default())
            .unwrap_or(self.key_up.len());
        let mut key_down = self
            .key_down
            .iter()
            .position(|k| k == &Key::default())
            .unwrap_or(self.key_down.len());

        for event in self.pump.poll_iter() {
            match event {
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_up >= self.mouse_up.len() {
                        sdl3::log::log_warn(
                            sdl3::log::Category::Input,
                            "mouse_up buffer is full, dropping event",
                        );
                        continue;
                    }

                    self.mouse_up[mouse_up] = Mouse {
                        pos: ScreenPoint::new(x, y),
                        btn: mouse_btn,
                        clicks: 0,
                    };
                    mouse_up += 1;
                }
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn,
                    clicks,
                    ..
                } => {
                    if mouse_down >= self.mouse_down.len() {
                        sdl3::log::log_warn(
                            sdl3::log::Category::Input,
                            "mouse_down buffer is full, dropping event",
                        );
                        continue;
                    }

                    self.mouse_down[mouse_down] = Mouse {
                        pos: ScreenPoint::new(x, y),
                        clicks,
                        btn: mouse_btn,
                    };
                    mouse_down += 1;
                }
                Event::KeyUp {
                    keycode, keymod, ..
                } => {
                    if key_up >= self.key_up.len() {
                        sdl3::log::log_warn(
                            sdl3::log::Category::Input,
                            "key_up buffer is full, dropping event",
                        );
                        continue;
                    }

                    let Some(key) = keycode else {
                        sdl3::log::log_warn(sdl3::log::Category::Input, "received unknown key");
                        continue;
                    };

                    self.key_up[key_up] = Key {
                        key,
                        key_mod: keymod,
                    };
                    key_up += 1;
                }
                Event::KeyDown {
                    keycode, keymod, ..
                } => {
                    if key_down >= self.key_down.len() {
                        sdl3::log::log_warn(
                            sdl3::log::Category::Input,
                            "key_down buffer is full, dropping event",
                        );
                        continue;
                    }
                    let Some(key) = keycode else {
                        sdl3::log::log_warn(sdl3::log::Category::Input, "received unknown key");
                        continue;
                    };

                    self.key_down[key_down] = Key {
                        key,
                        key_mod: keymod,
                    };
                    key_down += 1;
                }
                _ => continue,
            }
        }

        // Use defaults as sentinel
        if mouse_up < self.mouse_up.len() {
            self.mouse_up[mouse_up] = Mouse::default()
        }
        if mouse_down < self.mouse_down.len() {
            self.mouse_down[mouse_down] = Mouse::default()
        }
        if key_up < self.key_up.len() {
            self.key_up[key_up] = Key::default()
        }
        if key_down < self.key_down.len() {
            self.key_down[key_down] = Key::default()
        }
    }

    pub fn mouse_up(&self, btn: MouseButton) -> Option<&Mouse> {
        self.mouse_up.iter().find(|m| m.btn == btn)
    }

    pub fn mouse_down(&self, btn: MouseButton) -> Option<&Mouse> {
        self.mouse_down.iter().find(|m| m.btn == btn)
    }

    pub fn key_down(&self, key: Keycode) -> Option<&Key> {
        self.key_down.iter().find(|k| k.key == key)
    }

    pub fn key_up(&self, key: Keycode) -> Option<&Key> {
        self.key_up.iter().find(|k| k.key == key)
    }
}
