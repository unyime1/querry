use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{glib, CompositeTemplate, Image, Label};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/etim/querry/collection_row.ui")]
pub struct CollectionRow {
    #[template_child]
    pub collection_icon: TemplateChild<Image>,
    #[template_child]
    pub collection_label: TemplateChild<Label>,
    #[template_child]
    pub collection_menu: TemplateChild<Image>,
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// Trait shared by all GObjects
impl ObjectImpl for CollectionRow {}

// Trait shared by all widgets
impl WidgetImpl for CollectionRow {}

// Trait shared by all boxes
impl BoxImpl for CollectionRow {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CollectionRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "QuerryCollectionRow";
    type Type = super::CollectionRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
