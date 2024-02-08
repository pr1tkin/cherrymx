pub trait Effect {
    fn update_opacity(&self, time_elapsed: f64) -> f64;
}
