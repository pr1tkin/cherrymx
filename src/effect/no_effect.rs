use std::time::{Duration};
use gtk::cairo::{Context};
use crate::effect::effect::Effect;


pub struct NoEffect;

const DEFAULT_OPACITY: f64 = 0.5;

impl NoEffect {
    pub fn new() -> Self {
        NoEffect
    }
}

impl Effect for NoEffect {
    fn update_opacity(&self, _time_elapsed: f64) -> f64 {
        DEFAULT_OPACITY
    }
    fn draw(&self, _cr: &Context, _elapsed: Duration, _color: u32) {

    }
}
