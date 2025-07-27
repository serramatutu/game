//! Utilities to handle events

use sdl3::{
    EventPump,
    event::Event,
    keyboard::{Keycode, Mod},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Key {
    key: Keycode,
    key_mod: Mod,
}

impl Default for Key {
    fn default() -> Self {
        Self {
            key: Keycode::Unknown,
            key_mod: Mod::empty(),
        }
    }
}

pub struct Events {
    pump: EventPump,
    key_up: [Key; 64],
    key_down: [Key; 64],
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump,
            key_down: [Key::default(); 64],
            key_up: [Key::default(); 64],
        }
    }

    // Flush all current events
    pub fn clear(&mut self) {
        self.key_up[0] = Key::default();
        self.key_down[0] = Key::default();
    }

    /// Rescan the event pump for the newest events
    pub fn scan(&mut self) {
        let mut up = self
            .key_up
            .iter()
            .position(|k| k == &Key::default())
            .unwrap_or(self.key_up.len());
        let mut down = self
            .key_down
            .iter()
            .position(|k| k == &Key::default())
            .unwrap_or(self.key_down.len());

        for event in self.pump.poll_iter() {
            match event {
                Event::KeyUp {
                    keycode, keymod, ..
                } => {
                    if up >= self.key_up.len() {
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

                    self.key_up[up] = Key {
                        key,
                        key_mod: keymod,
                    };
                    up += 1;
                }
                Event::KeyDown {
                    keycode, keymod, ..
                } => {
                    if down >= self.key_down.len() {
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

                    self.key_down[down] = Key {
                        key,
                        key_mod: keymod,
                    };
                    down += 1;
                }
                _ => continue,
            }
        }

        // Use Key::default as sentinel
        if up < self.key_up.len() {
            self.key_up[up] = Key::default()
        }
        if down < self.key_down.len() {
            self.key_down[down] = Key::default()
        }
    }

    pub fn key_down(&self, key: Keycode) -> Option<&Key> {
        self.key_down.iter().find(|k| k.key == key)
    }

    pub fn key_up(&self, key: Keycode) -> Option<&Key> {
        self.key_up.iter().find(|k| k.key == key)
    }
}
