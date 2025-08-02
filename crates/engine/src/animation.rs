#[derive(Clone)]
pub struct Keyframe<T: Clone> {
    duration_ms: u16,
    end_at_ms: u64,
    value: T,
}

impl<T: Clone> Keyframe<T> {
    pub fn new(duration_ms: u16, value: T) -> Self {
        Self {
            duration_ms,
            value,
            end_at_ms: 0,
        }
    }
}

#[derive(Clone)]
pub struct Animation<T: Clone> {
    started_ms: u64,
    keyframes: Vec<Keyframe<T>>,
    pub cursor: usize,
}

impl<T: Clone> Animation<T> {
    /// Whether this animation is playing
    pub fn playing(&self) -> bool {
        self.cursor < self.keyframes.len()
    }

    /// Create a new animation from keyframe durations
    pub fn new(keyframes: &[Keyframe<T>]) -> Self {
        assert!(!keyframes.is_empty(), "Empty keyframes");
        Self {
            started_ms: 0,
            cursor: keyframes.len(),
            keyframes: Vec::from(keyframes),
        }
    }

    /// Start playing this animation
    pub fn start(&mut self, now_ms: u64) -> &T {
        self.started_ms = now_ms;
        self.cursor = 0;

        let mut acc = now_ms;
        for k in self.keyframes.iter_mut() {
            k.end_at_ms = acc + k.duration_ms as u64;
            acc = k.end_at_ms;
        }

        &self.keyframes.first().unwrap().value
    }

    /// Update this animation
    pub fn update(&mut self, now_ms: u64) -> Option<&T> {
        while self.cursor < self.keyframes.len() && self.keyframes[self.cursor].end_at_ms < now_ms {
            self.cursor += 1;
        }

        if self.playing() {
            Some(&self.keyframes[self.cursor].value)
        } else {
            None
        }
    }
}
