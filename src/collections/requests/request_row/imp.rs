use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{glib, CompositeTemplate, Image, Label, MenuButton};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/etim/querry/request_row.ui")]
pub struct RequestRow {
    #[template_child]
    pub request_id: TemplateChild<Label>,
    #[template_child]
    pub name: TemplateChild<Label>,
    #[template_child]
    pub request_icon: TemplateChild<Image>,
    pub bindings: RefCell<Vec<Binding>>,
    #[template_child]
    pub request_menu: TemplateChild<MenuButton>,
}

// Trait shared by all GObjects
impl ObjectImpl for RequestRow {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();
        let obj = self.obj();
        obj.process_hover();
    }
}

// Trait shared by all widgets
impl WidgetImpl for RequestRow {}

// Trait shared by all boxes
impl BoxImpl for RequestRow {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RequestRow {
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
