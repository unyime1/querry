use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, Box, CompositeTemplate};

// Initialize composite template for Window.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/etim/querry/requests_view.ui")]
pub struct RequestsView {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RequestsView {
    const NAME: &'static str = "QuerryRequestsView";
    type Type = super::RequestsView;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for RequestsView {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();

        let obj = self.obj();
    }
}

// Trait shared by all widgets
impl WidgetImpl for RequestsView {}

impl BoxImpl for RequestsView {}
