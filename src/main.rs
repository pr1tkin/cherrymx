use std::rc::Rc;
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use gtk::{glib};
use cherrymx::app_state::AppState;
use cherrymx::app_main_ui;
use cherrymx::service::hotplug::{ setup_hotplug};
use cherrymx::util::css_provider::setup_css_provider;

fn main() -> glib::ExitCode {
    let app = gtk::Application::new(Some("org.pritkin.cherrymx"), Default::default());
    let (hotplug_handle, hotplug_sender, hotplug_receiver) = setup_hotplug().expect("Failed to setup hotplug");
    let hotplug_receiver = Arc::new(Mutex::new(hotplug_receiver));

    app.connect_activate(move |app| {
        let app_state = Rc::new(AppState::new());
        setup_css_provider();
        let hotplug_receiver_clone = hotplug_receiver.clone();
        app_main_ui::setup_ui(app, app_state, hotplug_receiver_clone);
    });

    app.connect_shutdown(move |_| {
        hotplug_sender.send(()).expect("Failed to send termination signal to hotplug thread");
    });

    let exit_code = app.run();
    hotplug_handle.join().expect("Failed to join hotplug thread");

    std::process::exit(i32::from(exit_code));
}