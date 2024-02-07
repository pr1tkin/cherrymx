use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ColorButton, Grid, Label, Orientation, pango, Separator};
use gtk::Align::Fill;
use crate::app_state::AppState;
use crate::service::mxlp21_keyboard::{find_mxlp21_keyboard, set_values};
use crate::util::color::rgba_to_hex;
use crate::ui::dialog::create_device_not_detected_dialog;
use crate::widgets::buttons::{create_button_label, create_buttons};
use crate::widgets::keyboard::create_keyboard_image;
use crate::widgets::main_content::create_main_content_box;
use crate::widgets::scale::create_scale_widget;

pub fn setup_ui(app: &Application, app_state: Rc<AppState>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cherry Utility")
        .default_width(1440)
        .default_height(796)
        .build();

    let grid = Grid::builder()
        .row_spacing(0)
        .column_spacing(0)
        .build();

    grid.set_column_homogeneous(false);
    grid.set_vexpand(true);
    grid.set_hexpand(true);
    grid.set_css_classes(&["grid-layer"]);
    grid.set_valign(Fill);

    let dialog = create_device_not_detected_dialog(&window);

    match find_mxlp21_keyboard() {
        Some(device) => {
            let image_box = gtk::Box::new(Orientation::Vertical, 0);
            image_box.set_css_classes(&["image-container"]);
            image_box.set_vexpand(true);

            let keyboard_image = create_keyboard_image();
            image_box.append(&keyboard_image);

            let label = Label::new(Some("MX-LP 2.1 COMPACT WIRELESS"));
            label.set_wrap(true);  // Enable word-wrapping within the label
            label.set_wrap_mode(pango::WrapMode::WordChar); // Wrap on words or characters if necessary
            label.set_max_width_chars(24); // Set a maximum width in characters for the label before it wraps
            label.set_justify(gtk::Justification::Center);
            label.set_css_classes(&["image-box-label"]);
            image_box.append(&label);

            let separator = Separator::new(gtk::Orientation::Horizontal);
            separator.set_opacity(1.0);
            separator.set_css_classes(&["image-separator"]);
            image_box.append(&separator);

            grid.attach(&image_box, 0, 0, 1, 2);

            let label = create_button_label();
            grid.attach(&label, 1, 0, 1, 1);

            let button_box = create_buttons(&app_state);

            let lighting_color = Label::new(Some("Lighting Color"));
            lighting_color.set_css_classes(&["lighting-colorpicker-title"]);
            lighting_color.set_justify(gtk::Justification::Left);
            button_box.append(&lighting_color);

            // Color chooser
            let color_button = ColorButton::new();
            let app_state_clone = app_state.clone();
            color_button.connect_color_set(move |color_button| {
                let gdk_rgba = color_button.rgba();
                let red = (gdk_rgba.red() * 255.0) as u32;
                let green = (gdk_rgba.green() * 255.0) as u32;
                let blue = (gdk_rgba.blue() * 255.0) as u32;
                let alpha = (gdk_rgba.alpha() * 255.0) as u32;

                let color_value = (alpha << 24) | (red << 16) | (green << 8) | blue;
                app_state_clone.update_color(color_value);
                app_state_clone.drawing_area.queue_draw();
            });
            button_box.append(&color_button);

            let separator = Separator::new(gtk::Orientation::Horizontal);
            button_box.append(&separator);

            let speed_label = Label::new(Some("Speed"));
            speed_label.set_css_classes(&["button-speed"]);
            speed_label.set_justify(gtk::Justification::Left);
            button_box.append(&speed_label);

            let scale = create_scale_widget(&app_state);
            button_box.append(&scale);

            let button_apply = Button::with_label("Apply");
            button_apply.set_css_classes(&["button-apply"]);

            let color_button_clone = color_button.clone();
            let app_state_clone = app_state.clone();
            button_apply.connect_clicked(move |_| {
                let rgba = color_button_clone.rgba();
                let hex = rgba_to_hex(rgba);
                let color: u32 = u32::from_str_radix(&*hex, 16).unwrap_or_else(|_| {
                    eprintln!("Invalid color format. Please provide a hex color code without the '#'.");
                    0
                });
                set_values(&device, color, *app_state_clone.mode_value.borrow(), *app_state_clone.scale_value.borrow());
            });

            button_box.append(&button_apply);
            grid.attach(&button_box, 1, 1, 1, 1);

            let main_content = create_main_content_box(&app_state);
            grid.attach(&main_content, 2, 1, 1, 2);

            window.set_child(Some(&grid));
        } None => {
                dialog.show();
            }
        }
    window.show();
}