use gtk::{Dialog, Label, Button, pango, ApplicationWindow};
use gtk::prelude::{ BoxExt, ButtonExt, DialogExt, GtkWindowExt, OrientableExt, WidgetExt};

pub fn create_device_not_detected_dialog(parent: &ApplicationWindow) -> Dialog {
    let dialog = Dialog::builder()
        .title("CHERRY device not detected!")
        .transient_for(parent)
        .default_width(800)
        .default_height(350)
        .css_name("dialog")
        .modal(true)
        .build();

    let content_area = dialog.content_area();
    dialog.show();
    content_area.set_orientation(gtk::Orientation::Vertical);
    content_area.set_valign(gtk::Align::Center);
    content_area.set_margin_bottom(20);
    content_area.set_margin_top(20);
    content_area.set_margin_start(20);
    content_area.set_margin_end(20);

    let title_label = Label::new(None);
    title_label.set_markup("<b>CHERRY device not detected!</b>");
    title_label.set_css_classes(&["title-label"]);

    title_label.set_wrap(true);
    title_label.set_wrap_mode(pango::WrapMode::WordChar);
    title_label.set_max_width_chars(30);
    title_label.set_justify(gtk::Justification::Center);

    content_area.append(&title_label);

    let label = Label::new(Some("Please make sure the driver supports the device, then check the USB connection between the device and the computer"));
    label.set_css_classes(["message-label"].as_ref());

    label.set_wrap(true);
    label.set_wrap_mode(pango::WrapMode::WordChar);
    label.set_max_width_chars(30);
    label.set_justify(gtk::Justification::Center);

    content_area.append(&label);

    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    button_box.set_halign(gtk::Align::Center);
    button_box.set_valign(gtk::Align::End);
    button_box.set_margin_top(12);

    let button = Button::with_label("Close");
    button.set_css_classes(&["destructive-action"]);

    button.connect_clicked(move |_| {
        println!("Close button clicked!");
        std::process::exit(i32::from(1));
    });
    button_box.append(&button);

    content_area.append(&button_box);
    dialog.set_deletable(false);

    dialog
}