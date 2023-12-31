use std::cell::RefCell;

use adw::{prelude::*, subclass::prelude::*};
use glib::subclass::InitializingObject;
use gtk::{
    glib::{self, Properties},
    Box, CompositeTemplate, Image, Label,
};

// Initialize composite template for Window.
#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::CollectionItem)]
#[template(resource = "/org/etim/querry/collection_item.ui")]
pub struct CollectionItem {
    #[property(get, set)]
    pub name: RefCell<String>,
    #[property(get, set)]
    pub id: RefCell<String>,
    #[property(get, set)]
    pub icon_name: RefCell<String>,

    #[template_child]
    pub collection_icon: TemplateChild<Image>,
    #[template_child]
    pub collection_label: TemplateChild<Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CollectionItem {
    const NAME: &'static str = "QuerryCollectionItem";
    type Type = super::CollectionItem;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for CollectionItem {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();

        let obj = self.obj();
        obj.set_collection_name();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CollectionItem {}

impl BoxImpl for CollectionItem {}
