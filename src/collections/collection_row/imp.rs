use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{gio, glib, CompositeTemplate, Image, Label, ListView, MenuButton};
use once_cell::sync::OnceCell;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/etim/querry/collection_row.ui")]
pub struct CollectionRow {
    #[template_child]
    pub requests_list: TemplateChild<ListView>,
    pub requests_store: OnceCell<gio::ListStore>,

    #[template_child]
    pub collection_icon: TemplateChild<Image>,
    #[template_child]
    pub collection_label: TemplateChild<Label>,
    #[template_child]
    pub collection_menu: TemplateChild<MenuButton>,

    pub collection_id: RefCell<String>,
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// Trait shared by all GObjects
impl ObjectImpl for CollectionRow {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_requests();
    }
}

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

        klass.install_action_async(
            "col.remove-collection",
            None,
            |collection_row, _, _| async move {
                collection_row.delete_collection().await;
            },
        );

        klass.install_action("col.add-http-request", None, |collection_row, _, _| {
            collection_row.create_http_request_item();
        });
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
