use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{glib, Box, CompositeTemplate, Label};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/etim/querry/request_row.ui")]
pub struct RequestRow {
    pub request_id: RefCell<String>,
    #[template_child]
    pub name: TemplateChild<Label>,
    #[template_child]
    pub protocol: TemplateChild<Label>,
    #[template_child]
    pub request_icon_box: TemplateChild<Box>,
    #[template_child]
    pub http_method: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

// Trait shared by all GObjects
impl ObjectImpl for RequestRow {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_display();
    }
}

// Trait shared by all widgets
impl WidgetImpl for RequestRow {}

// Trait shared by all boxes
impl BoxImpl for RequestRow {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RequestRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "QuerryRequestRow";
    type Type = super::RequestRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
