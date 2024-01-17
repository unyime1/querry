mod collections;
mod database;
mod rest;
mod utils;
mod window;

use adw::{prelude::*, Application};
use gtk::{gdk::Display, gio, glib, CssProvider};

use database::{get_database, migrate_database};
use window::Window;

const APP_ID: &str = "org.etim.querry";

fn main() -> glib::ExitCode {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Can't subscribe");

    gio::resources_register_include!("querry.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    let database_connection = get_database().expect("could not get db");

    migrate_database(&database_connection).expect("Migrations failed.");

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);

    window.maximize();
    window.present();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/org/etim/querry/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
