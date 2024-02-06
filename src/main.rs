use gtk::prelude::*;
use gtk::{ Application, glib};
use cherrymx::app_state::AppState;
use cherrymx::app_main_ui;
use cherrymx::util::css_provider::setup_css_provider;


const APP_ID: &str = "org.pritkin.cherrymx";
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        let app_state = AppState::new();
        setup_css_provider();
        app_main_ui::setup_ui(app, app_state);
    });

    app.run()
}