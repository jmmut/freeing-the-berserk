pub struct Action {
    start_s_ago: Option<f32>,
    duration: f32,
}

impl Action {
    pub fn new(duration: f32) -> Self {
        Self {
            start_s_ago: None,
            duration,
        }
    }
    pub fn tick(&mut self, delta_s: f64) {
        if let Some(start_s_ago) = &mut self.start_s_ago {
            *start_s_ago += delta_s as f32;
            if *start_s_ago > self.duration {
                self.start_s_ago = None;
            }
        }
    }
    pub fn is_ongoing(&self) -> bool {
        self.start_s_ago.is_some()
    }
    pub fn ratio(&self) -> Option<f32> {
        self.start_s_ago.map(|s| s / self.duration)
    }
    pub fn stop(&mut self) {
        self.start_s_ago = None;
    }
    pub fn start(&mut self) {
        self.start_s_ago = Some(0.0);
    }
    pub fn duration(&self) -> f32 {
        self.duration
    }
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
        self.tick(0.0);
    }
}
