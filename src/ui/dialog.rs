use gtk::{Dialog, Label, Button, pango, ApplicationWindow};
use gtk::prelude::{BoxExt, DialogExt, OrientableExt, WidgetExt};

pub fn create_device_not_detected_dialog(parent: &ApplicationWindow) -> Dialog {
    let dialog = Dialog::builder()
        .title("CHERRY device not detected!")
        .transient_for(parent)
        .default_width(800)
        .default_height(350)
        .css_name("dialog")
        .modal(true)
        .build();

    // Add a label to the dialog's content area
    let content_area = dialog.content_area();
    content_area.set_orientation(gtk::Orientation::Vertical);
    content_area.set_valign(gtk::Align::Center);
    content_area.set_margin_bottom(20);
    content_area.set_margin_top(20);
    content_area.set_margin_start(20);
    content_area.set_margin_end(20);

    let title_label = Label::new(None);
    title_label.set_markup("<b>CHERRY device not detected!</b>");
    title_label.set_css_classes(&["title-label"]);

    title_label.set_wrap(true);  // Enable word-wrapping within the title_label
    title_label.set_wrap_mode(pango::WrapMode::WordChar); // Wrap on words or characters if necessary
    title_label.set_max_width_chars(30); // Set a maximum width in characters for the title_label before it wraps
    title_label.set_justify(gtk::Justification::Center); // Justify the text to be centered


    content_area.append(&title_label);

    let label = Label::new(Some("Please make sure the driver supports the device, then check the USB connection between the device and the computer"));
    label.set_css_classes(["message-label"].as_ref());

    label.set_wrap(true);  // Enable word-wrapping within the label
    label.set_wrap_mode(pango::WrapMode::WordChar); // Wrap on words or characters if necessary
    label.set_max_width_chars(30); // Set a maximum width in characters for the label before it wraps
    label.set_justify(gtk::Justification::Center); // Justify the text to be centered

    content_area.append(&label); // gtk4 method to add widgets

    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    button_box.set_halign(gtk::Align::Center); // Align the box itself to the center horizontally
    button_box.set_valign(gtk::Align::End); // Align the box to the end vertically
    button_box.set_margin_top(12); // Space above the box

    // Create the button and add it to the button_box
    let button = Button::with_label("Close");
    // button.set_vexpand(true);
    button.set_css_classes(&["destructive-action"]);
    // button.set_hexpand(true); // The button will expand to fill the box
    button_box.append(&button); // Add the button to the box

    // Add the button_box to the content_area
    content_area.append(&button_box);





    dialog
}