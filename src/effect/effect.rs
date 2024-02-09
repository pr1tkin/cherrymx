use std::time::{Duration};
use gtk::cairo::Context;

pub trait Effect {
    fn update_opacity(&self, time_elapsed: f64) -> f64;
    fn draw(&self, cr: &Context, elapsed: Duration, color: u32);
}
