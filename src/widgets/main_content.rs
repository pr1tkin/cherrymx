use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Button, gdk_pixbuf, Label, Orientation, pango, Picture};
use gtk::prelude::{BoxExt, WidgetExt};

pub fn create_main_content_box() -> gtk::Box {
    let large_content_box = gtk::Box::new(Orientation::Vertical, 0);
    large_content_box.set_css_classes(&["content-container"]);

    let pixbuf = Pixbuf::from_file("keyboard-large.png")
        .expect("Could not load the image file");
    let scaled_pixbuf = pixbuf.scale_simple(715, 232, gdk_pixbuf::InterpType::Bilinear)
        .expect("Could not scale the image");
    let picture = Picture::for_pixbuf(&scaled_pixbuf);
    picture.set_can_shrink(false);
    picture.set_keep_aspect_ratio(true);
    picture.set_css_classes(&["image-large-keyboard"]);

    large_content_box.append(&picture);

    let label4 = Label::new(Some("MX-LP 2.1 Compact Wireless Mechanical Keyboard"));
    label4.set_css_classes(&["product-title"]);
    label4.set_wrap(true);  // Enable word-wrapping within the label
    label4.set_wrap_mode(pango::WrapMode::WordChar); // Wrap on words or characters if necessary
    label4.set_max_width_chars(15); // Set a maximum width in characters for the label before it wraps
    label4.set_justify(gtk::Justification::Center);
    large_content_box.append(&label4);

    let large_content = Button::with_label("KB Setting");
    large_content.set_css_classes(&["button-Settings"]);
    large_content_box.append(&large_content);

    large_content_box
}