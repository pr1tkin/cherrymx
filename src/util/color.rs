use std::num::ParseIntError;
use gtk::gdk::RGBA;

pub fn rgba_to_hex(rgba: RGBA) -> String {
    let red = (rgba.red() * 255.0) as u8;
    let green = (rgba.green() * 255.0) as u8;
    let blue = (rgba.blue() * 255.0) as u8;

    format!("{:02X}{:02X}{:02X}", red, green, blue)
}

pub fn hex_to_rgba(hex_color: &u32) -> Result<RGBA, ParseIntError> {
    let r = ((hex_color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex_color >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex_color & 0xFF) as f32 / 255.0;

    Ok(RGBA::new(r, g, b, 1.0))
}