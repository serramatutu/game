use allocator_api2::vec::Vec;
use serde::{Deserialize, Serialize};

/// A keyframe in an animation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Keyframe<T: Clone> {
    pub duration_ms: u16,
    pub cumulative_duration_ms: u16,
    pub value: T,
}

impl<T: Clone> Keyframe<T> {
    pub fn new(duration_ms: u16, value: T) -> Self {
        Self {
            duration_ms,
            value,
            cumulative_duration_ms: 0,
        }
    }
}

/// A collection of keyframes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Animation<T: Clone> {
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Clone> Animation<T> {
    /// Create a new animation from keyframes
    pub fn new(keyframes: impl IntoIterator<Item = Keyframe<T>>) -> Self {
        let kf: Vec<_> = keyframes
            .into_iter()
            .scan(0u16, |acc, k| {
                *acc += k.duration_ms;
                Some(Keyframe {
                    duration_ms: k.duration_ms,
                    cumulative_duration_ms: *acc,
                    value: k.value,
                })
            })
            .collect();
        assert!(!kf.is_empty(), "Empty keyframes");
        Self { keyframes: kf }
    }
}

/// A playback cursor in an animation
#[derive(Copy, Clone, Debug, Default)]
pub struct AnimationCursor {
    start_ms: u64,
    current_frame: usize,
    pub playing: bool,
}

impl AnimationCursor {
    /// Create a new cursor
    pub fn new() -> Self {
        Self::default()
    }

    /// Start this cursor
    pub fn start<'anim, T: Clone>(
        &mut self,
        now_ms: u64,
        animation: &'anim Animation<T>,
    ) -> &'anim T {
        self.start_ms = now_ms;
        self.current_frame = 0;
        self.playing = true;

        &animation
            .keyframes
            .first()
            .expect("Keyframes cannnot be empty")
            .value
    }

    /// Update this cursor
    pub fn update<'anim, T: Clone>(
        &mut self,
        now_ms: u64,
        animation: &'anim Animation<T>,
    ) -> Option<&'anim T> {
        if !self.playing {
            return None;
        }

        while self.current_frame < animation.keyframes.len()
            && animation.keyframes[self.current_frame].cumulative_duration_ms as u64 + self.start_ms
                < now_ms
        {
            self.current_frame += 1;
        }

        if self.current_frame < animation.keyframes.len() {
            Some(&animation.keyframes[self.current_frame].value)
        } else {
            self.playing = false;
            None
        }
    }
}
