use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, Box, CompositeTemplate, EditableLabel, Entry, Label, MenuButton, Separator};

use super::HTTPMethods;

// Initialize composite template for Window.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/etim/querry/requests_view.ui")]
pub struct RequestsView {
    #[template_child]
    pub collection_name: TemplateChild<Label>,
    #[template_child]
    pub divider: TemplateChild<Label>,
    #[template_child]
    pub request_name: TemplateChild<EditableLabel>,
    pub request_id: RefCell<String>,
    pub collection_id: RefCell<String>,
    #[template_child]
    pub actions_box: TemplateChild<Box>,
    #[template_child]
    pub names_box: TemplateChild<Box>,
    #[template_child]
    pub separator: TemplateChild<Separator>,
    #[template_child]
    pub separator_2: TemplateChild<Separator>,
    #[template_child]
    pub requests_menu: TemplateChild<MenuButton>,
    #[template_child]
    pub entry_box: TemplateChild<Entry>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RequestsView {
    const NAME: &'static str = "QuerryRequestsView";
    type Type = super::RequestsView;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        klass.install_action_async(
            "req.set-method-post",
            None,
            |request_view, _, _| async move {
                request_view.update_request_method(HTTPMethods::Post).await;
            },
        );

        klass.install_action_async(
            "req.set-method-get",
            None,
            |request_view, _, _| async move {
                request_view.update_request_method(HTTPMethods::Get).await;
            },
        );

        klass.install_action_async(
            "req.set-method-put",
            None,
            |request_view, _, _| async move {
                request_view.update_request_method(HTTPMethods::Put).await;
            },
        );

        klass.install_action_async(
            "req.set-method-delete",
            None,
            |request_view, _, _| async move {
                request_view
                    .update_request_method(HTTPMethods::Delete)
                    .await;
            },
        );
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
        obj.listen_request_view();
        obj.montitor_request_name_changes();
        obj.monitor_url_changes();
    }
}

// Trait shared by all widgets
impl WidgetImpl for RequestsView {}

impl BoxImpl for RequestsView {}
