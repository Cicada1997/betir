use macroquad::prelude::*;

pub struct Timer {
    pub start_time: f64,
    pub duration: f64,
    pub enabled: bool,
}

impl Timer {
    pub fn new(duration: f64, enabled: bool) -> Self {
        if duration.is_nan() || duration == 0. { panic!("Duration of timer can not be 0."); }

        Timer {
            start_time: get_time(),
            duration,
            enabled,
        }
    }
    pub fn start(&mut self) {
        self.enabled = true;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn reset(&mut self) {
        self.start_time = get_time();
    }

    pub fn is_finished(&self) -> bool {
        get_time() - self.start_time >= self.duration
    }

    pub fn elapsed(&self) -> f64 {
        get_time() - self.start_time
    }

    pub fn remaining(&self) -> f64 {
        (self.duration - (get_time() - self.start_time)).max(0.0)
    }

    pub fn percentage_elapsed(&self) -> f64 {
        self.elapsed() / self.duration
    }

    pub fn percentage_remaining(&self) -> f64 {
        self.remaining() / self.duration
    }
}
