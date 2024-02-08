use gtk::prelude::*;
use gtk::{Scale, Adjustment};
use crate::app_state::AppState;

pub fn create_scale_widget(app_state: &AppState) -> Scale {
    let adjustment = Adjustment::new(2.0, 0.0, 4.0, 1.0, 1.0, 0.0);
    let scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
    scale.set_digits(0);
    scale.set_hexpand(true);
    scale.set_value_pos(gtk::PositionType::Top);

    let scale_value = app_state.scale_value.clone();
    scale.connect_value_changed(move |s| {
        *scale_value.borrow_mut() = match s.value() as u8 {
            0 => 0x00,
            1 => 0x01,
            2 => 0x02,
            3 => 0x03,
            4 => 0x04,
            _ => 0x00,
        };
        println!("Selected scale value: {:02x}", *scale_value.borrow());
    });

    let initial_scale_value = *app_state.scale_value.borrow();
    if initial_scale_value != 0 {
        scale.set_value(initial_scale_value as f64);
    }

    scale
}