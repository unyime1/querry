use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::glib;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::RequestItem)]
pub struct RequestItem {
    #[property(get, set)]
    pub id: RefCell<String>,
    #[property(get, set)]
    pub name: RefCell<String>,
    #[property(get, set)]
    pub protocol: RefCell<String>,
    #[property(get, set)]
    pub httpmethod: RefCell<String>,
    #[property(get, set)]
    pub url: RefCell<String>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RequestItem {
    const NAME: &'static str = "QuerryRequestItem";
    type Type = super::RequestItem;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for RequestItem {}
