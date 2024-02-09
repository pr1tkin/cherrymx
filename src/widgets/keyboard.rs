use gtk::{gdk_pixbuf, Picture};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::WidgetExt;

pub fn create_keyboard_image() -> Picture {
    let pixbuf = Pixbuf::from_file("src/resource/keyboard-large.png").expect("Could not load the image file");
    let scaled_pixbuf = pixbuf.scale_simple(117, 45, gdk_pixbuf::InterpType::Bilinear).expect("Could not scale the image");
    let picture = Picture::for_pixbuf(&scaled_pixbuf);
    picture.set_can_shrink(true);
    picture.set_keep_aspect_ratio(true);
    picture.set_css_classes(&["image-small-keyboard"]);

    picture
}