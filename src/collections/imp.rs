use once_cell::sync::OnceCell;

use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, Box, CompositeTemplate, ListBox, gio};

// Initialize composite template for Window.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/etim/querry/collection.ui")]
pub struct CollectionsWindow {
    #[template_child]
    pub collections_list: TemplateChild<ListBox>,
    pub collections: OnceCell<gio::ListStore>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CollectionsWindow {
    const NAME: &'static str = "QuerryCollectionsWindow";
    type Type = super::CollectionsWindow;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for CollectionsWindow {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CollectionsWindow {}

impl BoxImpl for CollectionsWindow {}
