use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use glib::ControlFlow;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ColorButton, Dialog, glib, Grid, Label, Orientation, pango, Separator};
use gtk::Align::Fill;
use crate::app_state::AppState;
use crate::service::hotplug::{HotplugMessage};
use crate::service::mxlp21_keyboard::{find_mxlp21_keyboard, set_values};
use crate::util::color::{hex_to_rgba, rgba_to_hex};
use crate::ui::dialog::create_device_not_detected_dialog;
use crate::util::settings::save_settings;
use crate::widgets::buttons::{create_button_label, create_buttons};
use crate::widgets::keyboard::create_keyboard_image;
use crate::widgets::main_content::create_main_content_box;
use crate::widgets::scale::create_scale_widget;



pub fn setup_ui(app: &Application, app_state: Rc<AppState>, hotplug_receiver: Arc<Mutex<Receiver<HotplugMessage>>>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cherry Utility")
        .default_width(1440)
        .default_height(796)
        .build();

    let grid = setup_grid();
    let dialog = create_device_not_detected_dialog(&window);

    process_hotplug_messages(hotplug_receiver, dialog.clone());

    setup_keyboard_ui(&grid, &app_state, &dialog);

    window.set_child(Some(&grid));
    window.show();
}


fn process_hotplug_messages(hotplug_receiver: Arc<Mutex<Receiver<HotplugMessage>>>, dialog: Dialog) {
    let dialog_clone = dialog.clone();
    glib::idle_add_local(move || {
        let receiver = hotplug_receiver.lock().expect("Failed to lock receiver");
        match receiver.try_recv() {
            Ok(message) => {
                match message {
                    HotplugMessage::DeviceArrived(_) => {
                        dialog_clone.hide();
                    },
                    HotplugMessage::DeviceLeft(_) => {
                        dialog_clone.show();
                    },
                }
            },
            Err(_) => {
            }
        }
        ControlFlow::Continue
    });
}

fn create_keyboard_image_box() -> gtk::Box {
    let image_box = gtk::Box::new(Orientation::Vertical, 0);
    image_box.set_css_classes(&["image-container"]);
    image_box.set_vexpand(true);

    let keyboard_image = create_keyboard_image();
    image_box.append(&keyboard_image);

    let label = Label::new(Some("MX-LP 2.1 COMPACT WIRELESS"));
    label.set_wrap(true);
    label.set_wrap_mode(pango::WrapMode::WordChar);
    label.set_max_width_chars(24);
    label.set_justify(gtk::Justification::Center);
    label.set_css_classes(&["image-box-label"]);
    image_box.append(&label);

    let separator = Separator::new(gtk::Orientation::Horizontal);
    separator.set_opacity(1.0);
    separator.set_css_classes(&["image-separator"]);
    image_box.append(&separator);

    image_box
}

fn setup_keyboard_ui(grid: &Grid, app_state: &Rc<AppState>, dialog: &Dialog) {
    let device_option = find_mxlp21_keyboard();
    let device_present = device_option.is_some();
    let device = device_option.unwrap();


    let image_box = create_keyboard_image_box();
    grid.attach(&image_box, 0, 0, 1, 2);

    let label = create_button_label();
    grid.attach(&label, 1, 0, 1, 1);

    let button_box = create_buttons(app_state.clone());

    let lighting_color = Label::new(Some("Lighting Color"));
    lighting_color.set_css_classes(&["lighting-colorpicker-title"]);
    lighting_color.set_justify(gtk::Justification::Left);
    button_box.append(&lighting_color);

    // Color chooser
    let color_button = ColorButton::new();
    let initial_color = *app_state.color_value.borrow();
    if initial_color != 0 {
        let initial_color = hex_to_rgba(&initial_color).expect("TODO: panic message");
        color_button.set_rgba(&initial_color);
    }

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
    let device_clone = device.clone();
    button_apply.connect_clicked(move |_| {
        let rgba = color_button_clone.rgba();
        let hex = rgba_to_hex(rgba);
        let color: u32 = u32::from_str_radix(&*hex, 16).unwrap_or_else(|_| {
            eprintln!("Invalid color format. Please provide a hex color code without the '#'.");
            0
        });
        set_values(&device_clone, color, *app_state_clone.mode_value.borrow(), *app_state_clone.scale_value.borrow());
        save_settings(&app_state_clone).unwrap();
    });
    button_box.append(&button_apply);
    grid.attach(&button_box, 1, 1, 1, 1);

    let main_content = create_main_content_box(app_state);
    grid.attach(&main_content, 2, 1, 1, 2);

    if !device_present {
        dialog.show();
    }
}

fn setup_grid() -> Grid {
    let grid = Grid::builder()
        .row_spacing(0)
        .column_spacing(0)
        .build();

    grid.set_column_homogeneous(false);
    grid.set_vexpand(true);
    grid.set_hexpand(true);
    grid.set_css_classes(&["grid-layer"]);
    grid.set_valign(Fill);
    grid
}