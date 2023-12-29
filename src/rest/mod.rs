mod imp;

use adw::prelude::ButtonExt;
use glib::Object;
use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct RestWindow(ObjectSubclass<imp::RestWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RestWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn activate_btn(&self) {
        let button = self.imp().button.clone();

        // Connect to "clicked" signal of `button`
        button.connect_clicked(move |button| {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
            println!("Hello world");
        });
    }
}
