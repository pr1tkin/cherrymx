// use cherrymx::commands::{get_command, Run, Successful};
// use cherrymx::mxlp21_keyboard::find_mxlp21_keyboard;
// use std::{env::args, process::ExitCode};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Dialog, gdk, glib, Label, pango};
use gtk::glib::clone;
use cherrymx::mxlp21_keyboard::find_mxlp21_keyboard;


const APP_ID: &str = "org.gtk_rs.HelloWorld1";
fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        // Create a new application window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Cherry Utility")
            .default_width(840)
            .default_height(520)
            .build();

        // Create a new dialog with no parent window (modal)
        let dialog = Dialog::builder()
            .title("CHERRY device not detected!")
            .transient_for(&window)
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
        button.set_vexpand(true);
        button.set_css_classes(&["destructive-action"]);
        button.set_hexpand(true); // The button will expand to fill the box
        button_box.append(&button); // Add the button to the box

        // Add the button_box to the content_area
        content_area.append(&button_box);

        button.connect_clicked(clone!(@weak app => move |_| {
            app.quit();
        }));

        // Style the dialog and button using CSS
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data("
   .dialog {
        border: 1px solid #000;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
        background: #fff;
        color: #000;
        padding: 32px 16px;
    }
    .title-label {
        font-weight: bold;
        font-size: 32px;
        margin: 32px 16px;
    }
    .message-label {
        font-size: 24px;
        margin: 24px 16px;
        color: #a5a3a3;
    }
    .destructive-action {
        background: #e94b78;
        color: #fff;
        padding: 24px 16px;
        border-radius: 16px;
    }");

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Error initializing GTK CSS provider."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        match find_mxlp21_keyboard() {
            Some(_) => {
                // If everything is okay, continue with the normal application flow.
                println!("No need to show the dialog, continue with app");
                // ... the rest of your application logic
            },
            None => {
                dialog.show();
            }
        }

        window.show();
    });

    // Connect to "activate" signal of `app`
    // app.connect_activate(build_ui);

    // Run the application
    app.run()
    // let device = find_mxlp21_keyboard()
    //     .expect("No MX-LP 2.1 Compact Wireless Mechanical Keyboard keyboard found, sorry!");
    // let args = args().skip(1).collect::<Vec<_>>();
    //
    // let command = get_command(&args);
    // let cmd_status = command.run(&device);
    //
    // if cmd_status.successful() {
    //     ExitCode::SUCCESS
    // } else {
    //     ExitCode::from(cmd_status as u8)
    // }
}

// fn build_ui(app: &Application) {
//     // Create a window and set the title
//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("My GTK App")
//         .build();
//
//  let fickmich = title_and_subtitle("arsch", "loch");
//     let root = gtk::Overlay::builder()
//         .hexpand(true)
//         .vexpand(true)
//         .build();
//     let button_box = make_button_box();
// button_box.append(&fickmich);
//     root.add_overlay(&button_box);
//         root.set_visible(true);
//     // let dialog = ColorChooserDialog::builder()
//     //     .modal(true)
//     //     .title("Choose Color")
//     //     .hide_on_close(true)
//     //     .build();
//     //
//     // dialog.show();
//     // Present window
//
//     let close_button = gtk::Button::builder()
//         .icon_name("window-close-symbolic")
//         .css_classes(["close-button"])
//         .build();
//
//     let label = gtk::Label::builder().build();
//
//     let grid = gtk::Grid::builder().column_spacing(10).build();
//     grid.attach(&close_button, 0, 0, 1, 1);
//     grid.attach(&label, 1, 0, 1, 1);
//
//
//     window.present();
// }
//
// fn make_button_box() -> gtk::Box {
//     let button_box = gtk::Box::builder()
//         .halign(gtk::Align::Center)
//         .valign(gtk::Align::End)
//         .margin_bottom(20)
//         .margin_top(20)
//         .margin_start(20)
//         .margin_end(20)
//         .build();
//     button_box.style_context().add_class("linked");
//     button_box
// }
//
// fn title_label(label: &str, class: &str) -> gtk::Label {
//     gtk::Label::builder()
//         .label(label)
//         .single_line_mode(true)
//         .css_classes([class])
//         .build()
// }
// pub fn title_and_subtitle(title: &str, subtitle: &str) -> gtk::Widget {
//     let vbox = gtk::Box::builder()
//         .orientation(gtk::Orientation::Vertical)
//         .valign(gtk::Align::Center)
//         .build();
//     vbox.append(&title_label(title, "title"));
//     vbox.append(&title_label(subtitle, "subtitle"));
//     vbox.upcast()
// }