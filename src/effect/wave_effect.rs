use std::time::{Duration, Instant};
use gtk::cairo::{Context};
use crate::effect::effect::Effect;

const DEFAULT_OPACITY: f64 = 0.5;

pub struct WaveEffect {
    pub amplitude: f64,
    pub frequency: f64,
    pub speed: f64,
    pub start_time: Instant,
}

impl WaveEffect {
    // Initialize the WaveEffect with a start time
    pub fn new(amplitude: f64, frequency: f64, speed: f64) -> Self {
        WaveEffect {
            amplitude,
            frequency,
            speed,
            start_time: Instant::now(),
        }
    }

    // Calculate the wave offset based on the elapsed time
    fn calculate_wave_offset(&self, elapsed: Duration) -> f64 {
        (elapsed.as_secs_f64() * self.speed % 1.0).fract()
    }
}

impl Effect for WaveEffect {
    fn update_opacity(&self, _time_elapsed: f64) -> f64 {
        DEFAULT_OPACITY
    }

    fn draw(&self, cr: &Context, elapsed: Duration, color: u32) {
        let key_rectangles: Vec<(f64, f64, f64, f64)> = vec![
            (78.0, 46.0, 35.0, 35.0),
            (113.0, 46.0, 35.0, 35.0),
            (148.0, 46.0, 35.0, 35.0),
            (183.0, 46.0, 35.0, 35.0),
            (218.0, 46.0, 35.0, 35.0),
            (253.0, 46.0, 35.0, 35.0),
            (288.0, 46.0, 35.0, 35.0),
            (323.0, 46.0, 35.0, 35.0),
            (358.0, 46.0, 35.0, 35.0),
            (393.0, 46.0, 35.0, 35.0),
            (428.0, 46.0, 35.0, 35.0),
            (463.0, 46.0, 35.0, 35.0),
            (498.0, 46.0, 35.0, 35.0),
            (533.0, 46.0, 70.0, 35.0),
            (603.0, 46.0, 35.0, 35.0),
            (78.0, 83.0, 53.0, 35.0),
            (131.0, 83.0, 35.0, 35.0),
            (166.0, 83.0, 35.0, 35.0),
            (201.0, 83.0, 35.0, 35.0),
            (236.0, 83.0, 35.0, 35.0),
            (271.0, 83.0, 35.0, 35.0),
            (306.0, 83.0, 35.0, 35.0),
            (341.0, 83.0, 35.0, 35.0),
            (376.0, 83.0, 35.0, 35.0),
            (411.0, 83.0, 35.0, 35.0),
            (446.0, 83.0, 35.0, 35.0),
            (481.0, 83.0, 35.0, 35.0),
            (516.0, 83.0, 35.0, 35.0),
            (551.0, 83.0, 60.0, 35.0),
            (604.0, 83.0, 38.0, 75.0),
            (560.0, 118.0, 40.0, 65.0),
            (604.0, 83.0, 35.0, 35.0),
            (78.0, 118.0, 62.0, 35.0),
            (140.0, 118.0, 35.0, 35.0),
            (175.0, 118.0, 35.0, 35.0),
            (210.0, 118.0, 35.0, 35.0),
            (245.0, 118.0, 35.0, 35.0),
            (280.0, 118.0, 35.0, 35.0),
            (315.0, 118.0, 35.0, 35.0),
            (350.0, 118.0, 35.0, 35.0),
            (385.0, 118.0, 35.0, 35.0),
            (420.0, 118.0, 35.0, 35.0),
            (455.0, 118.0, 35.0, 35.0),
            (490.0, 118.0, 35.0, 35.0),
            (525.0, 118.0, 35.0, 35.0),
            (604.0, 118.0, 35.0, 35.0),
            (78.0, 153.0, 43.0, 35.0),
            (121.0, 153.0, 35.0, 35.0),
            (156.0, 153.0, 35.0, 35.0),
            (191.0, 153.0, 35.0, 35.0),
            (226.0, 153.0, 35.0, 35.0),
            (261.0, 153.0, 35.0, 35.0),
            (296.0, 153.0, 35.0, 35.0),
            (331.0, 153.0, 35.0, 35.0),
            (366.0, 153.0, 35.0, 35.0),
            (401.0, 153.0, 35.0, 35.0),
            (436.0, 153.0, 35.0, 35.0),
            (471.0, 153.0, 35.0, 35.0),
            (506.0, 153.0, 62.0, 35.0),
            (568.0, 153.0, 35.0, 35.0),
            (603.0, 153.0, 35.0, 35.0),
            (78.0, 188.0, 43.0, 35.0),
            (121.0, 188.0, 43.0, 35.0),
            (164.0, 188.0, 43.0, 35.0),
            (207.0, 188.0, 220.0, 35.0),
            (427.0, 188.0, 35.0, 35.0),
            (462.0, 188.0, 35.0, 35.0),
            (497.0, 188.0, 35.0, 35.0),
            (532.0, 188.0, 35.0, 35.0),
            (567.0, 188.0, 35.0, 35.0),
            (602.0, 188.0, 35.0, 35.0),
        ];

        let wave_offset = self.calculate_wave_offset(elapsed);
        let base_color = 0xe0e0e0;
        let r = ((color >> 16) & 0xFF) as f64 / 255.0;
        let g = ((color >> 8) & 0xFF) as f64 / 255.0;
        let b = (color & 0xFF) as f64 / 255.0;

        let base_r = ((base_color >> 16) & 0xFF) as f64 / 255.0;
        let base_g = ((base_color >> 8) & 0xFF) as f64 / 255.0;
        let base_b = (base_color & 0xFF) as f64 / 255.0;

        for (x, y, width, height) in key_rectangles.iter() {
            let wave_phase = ((x / width) + wave_offset) * self.frequency;
            let intensity = ((wave_phase).sin() * self.amplitude).abs();

            let final_r = base_r + (r - base_r) * intensity;
            let final_g = base_g + (g - base_g) * intensity;
            let final_b = base_b + (b - base_b) * intensity;

            cr.set_source_rgba(final_r, final_g, final_b, 0.5);
            cr.rectangle(*x, *y, *width, *height);
            cr.fill().expect("Failed to draw rectangle");
        }
    }
}
