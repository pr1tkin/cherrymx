use gtk::{Button, CssProvider, gdk, Label, Orientation, Separator, STYLE_PROVIDER_PRIORITY_USER};
use gtk::prelude::{BoxExt, ButtonExt, StyleContextExt, WidgetExt};
use crate::app_state::AppState;

pub fn create_buttons(app_state: &AppState) -> gtk::Box {

    let button_labels = [
        "Wave", "Spectrum", "Breathing", "Rolling",
        "Curve", "Scan", "Radiation", "Ripples",
        "Single Key", "Static", "Custom",

    ];

    let button_mode = [0x00,0x01,0x02,0x0a,
        0x0c,0x0f,0x12,0x13,
        0x15,0x03,0x08];


    let button_box = gtk::Box::new(Orientation::Vertical, 16);
    button_box.set_css_classes(&["button-container"]);


    let provider = CssProvider::new();
    provider.load_from_data(
        ".active { background: #e94b78; color: #fff; }",
    );

    // Ensure the CSS provider is added to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_USER,
    );

    for i in (0..button_labels.len()).step_by(2) {
        let row_box = gtk::Box::new(Orientation::Horizontal, 0);
        for j in i..i+2 {
            if j < button_labels.len() {
                let mode_value = app_state.mode_value.clone();
                let button = Button::with_label(button_labels[j]);
                button.set_css_classes(&[&format!("color-button-{}", j)]);

                let buttons_clone = app_state.button_modes.clone();
                button.connect_clicked(move |clicked_button| {
                    *mode_value.borrow_mut() = button_mode[j];
                    for btn in buttons_clone.borrow().iter() {
                        btn.style_context().remove_class("active");
                        println!("Button: {:?}", btn);
                    }
                    clicked_button.style_context().add_class("active");
                });

                app_state.button_modes.borrow_mut().push(button.clone());
                row_box.append(&button);
            }
        }
        button_box.append(&row_box);
    }

    // Separator
    let separator = Separator::new(gtk::Orientation::Horizontal);
    button_box.append(&separator);

    button_box
}

pub fn create_button_label() -> gtk::Box {
    let label_box = gtk::Box::new(Orientation::Vertical, 0);
    label_box.set_css_classes(&["label-box"]);

    let label2 = Label::new(Some("Lighting"));
    label2.set_css_classes(&["lighting-title"]);
    label_box.append(&label2);

    let label3 = Label::new(Some("(Please apply the changes)"));
    label3.set_css_classes(&["lighting-subtitle"]);
    label_box.append(&label3);

    label_box
}