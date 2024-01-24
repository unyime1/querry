mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{gio::ListStore, glib, ListItem, ListView, SignalListItemFactory, SingleSelection};

use super::collection_item::CollectionItem;
use crate::database::get_database;
use crate::requests::{request_item::RequestItem, request_row::RequestRow};
use crate::utils::{
    crud::requests::{create_request, get_collection_requests, ProcolTypes},
    messaging::{AppEvent, EVENT_CHANNEL},
};

glib::wrapper! {
    pub struct CollectionRow(ObjectSubclass<imp::CollectionRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for CollectionRow {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Send a notification to delete specified collection.
    pub async fn delete_collection(&self) {
        let id = self.imp().collection_id.borrow().to_owned();

        EVENT_CHANNEL
            .0
            .send(AppEvent::CollectionDeleted(id))
            .await
            .expect("Channel should be open");
    }

    pub fn create_http_request_item(&self) {
        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };

        let collection_id = self.imp().collection_id.borrow();

        let request_data = match create_request(ProcolTypes::Http, collection_id.to_string(), &db) {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not create request.");
                return;
            }
        };

        let request_item = RequestItem::new(
            request_data.name,
            request_data.id,
            request_data.url,
            request_data.protocol,
            request_data.http_method,
        );
        self.get_requests_store().insert(0, &request_item)
    }

    pub fn bind(&self, collection_item: &CollectionItem) {
        // Get state
        let collection_icon = self.imp().collection_icon.get();
        let collection_label = self.imp().collection_label.get();

        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `collection_item.name` to `collection_row.collection_label.label`
        let collection_label_binding = collection_item
            .bind_property("name", &collection_label, "label")
            .bidirectional()
            .sync_create()
            .build();
        // Save binding
        bindings.push(collection_label_binding);

        // Bind `collection_item.icon` to `collection_row.collection_icon.icon_name`
        let collection_icon_binding = collection_item
            .bind_property("icon", &collection_icon, "icon-name")
            .sync_create()
            .build();
        // Save binding
        bindings.push(collection_icon_binding);

        self.set_collection_id(collection_item.id());
        self.setup_requests();
    }

    pub fn set_collection_id(&self, new_id: String) {
        *self.imp().collection_id.borrow_mut() = new_id;
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    pub fn get_requests_store(&self) -> ListStore {
        self.imp()
            .requests_store
            .get()
            .expect("requests not set.")
            .clone()
    }

    pub fn get_requests_list(&self) -> ListView {
        self.imp().requests_list.clone()
    }

    pub fn setup_requests(&self) {
        if self.imp().requests_store.get().is_some() {
            return;
        }

        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };

        let collection_id = self.imp().collection_id.borrow();
        let requests_vec = match get_collection_requests(&db, &collection_id) {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get collections requests.");
                Vec::new()
            }
        };

        let requests_model = ListStore::new::<RequestItem>();
        self.imp()
            .requests_store
            .set(requests_model.clone())
            .expect("Could not set requests.");

        let requests: Vec<RequestItem> = requests_vec
            .into_iter()
            .map(|request_item| {
                RequestItem::new(
                    request_item.name,
                    request_item.id,
                    request_item.url,
                    request_item.protocol,
                    request_item.http_method,
                )
            })
            .collect();

        self.get_requests_store().extend_from_slice(&requests);

        let factory = SignalListItemFactory::new();

        // Create an empty `RequestRow` during setup
        factory.connect_setup(move |_, list_item| {
            let request_row = RequestRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&request_row));
        });

        // Bind RequestRow to RequestItem
        factory.connect_bind(move |_, list_item| {
            // Get `RequestItem` from `ListItem`
            let request_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<RequestItem>()
                .expect("The item has to be an `RequestItem`.");

            // Get `RequestRow` from `ListItem`
            let request_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<RequestRow>()
                .expect("The child has to be a `RequestRow`.");

            request_row.bind(&request_item);
            request_row.set_request_id(request_item.id().to_string())
        });

        // Tell factory how to unbind `RequestRow` from `RequestItem`
        factory.connect_unbind(move |_, list_item| {
            let request_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<RequestRow>()
                .expect("The child has to be a `RequestRow`.");

            request_row.unbind();
        });

        let selection_model = SingleSelection::new(Some(self.get_requests_store()));
        self.get_requests_list().set_model(Some(&selection_model));
        self.get_requests_list().set_factory(Some(&factory));
        self.get_requests_list().set_single_click_activate(true);
        self.get_requests_list().set_css_classes(&vec!["collections_list"]);
    }
}
