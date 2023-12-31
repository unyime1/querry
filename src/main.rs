mod collections;
mod database;
mod entities;
mod rest;
mod utils;
mod window;

use adw::{prelude::*, Application};
use gtk::{
    gdk::Display,
    gio,
    glib::{self, clone},
    CssProvider,
};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use database::migrator::run_migrations;
use window::Window;

const APP_ID: &str = "org.etim.querry";
static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

fn main() -> glib::ExitCode {
    gio::resources_register_include!("querry.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    let (sender, receiver) = async_channel::bounded(1);
    RUNTIME.spawn(clone!(@strong sender => async move {
        let response = run_migrations().await;
        sender.send(response).await.expect("The channel needs to be open.");
    }));

    glib::spawn_future_local(async move {
        while let Ok(result) = receiver.recv().await {
            match result {
                Ok(data) => {
                    println!("{:?}", data)
                }
                Err(error) => {
                    println!("{:?}", error)
                }
            }
        }
    });

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
