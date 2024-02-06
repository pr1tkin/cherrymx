use gtk::gdk::RGBA;

pub fn rgba_to_hex(rgba: RGBA) -> String {
    let red = (rgba.red() * 255.0) as u8;
    let green = (rgba.green() * 255.0) as u8;
    let blue = (rgba.blue() * 255.0) as u8;

    format!("{:02X}{:02X}{:02X}", red, green, blue)
}