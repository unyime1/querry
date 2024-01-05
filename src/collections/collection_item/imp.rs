use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::glib;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CollectionItem)]
pub struct CollectionItem {
    #[property(get, set)]
    pub name: RefCell<String>,
    #[property(get, set)]
    pub id: RefCell<String>,
    #[property(get, set)]
    pub icon: RefCell<String>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CollectionItem {
    const NAME: &'static str = "QuerryCollectionItem";
    type Type = super::CollectionItem;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for CollectionItem {}
