use std::f64::consts::PI;
use std::time::{Duration};
use gtk::cairo::{Context};
use crate::effect::effect::Effect;

pub struct BreathingEffect {
    duration: f64,
}

impl BreathingEffect {
    pub fn new(duration: f64) -> Self {
        BreathingEffect { duration }
    }
}

impl Effect for BreathingEffect {
    fn update_opacity(&self, time_elapsed: f64) -> f64 {
        let t = (time_elapsed % self.duration) / self.duration;
        let opacity = ((t * 2.0 * PI).sin().powi(2) + 1.0) / 2.0;
        let scaled_opacity = opacity * 0.5;
        scaled_opacity
    }
    fn draw(&self, _cr: &Context, _elapsed: Duration, _color: u32) {

    }
}
