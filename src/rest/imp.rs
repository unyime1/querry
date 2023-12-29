use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, Box, CompositeTemplate};

// Initialize composite template for Window.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/etim/querry/rest.ui")]
pub struct RestWindow {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RestWindow {
    const NAME: &'static str = "QuerryRestWindow";
    type Type = super::RestWindow;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for RestWindow {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for RestWindow {}

impl BoxImpl for RestWindow {}
