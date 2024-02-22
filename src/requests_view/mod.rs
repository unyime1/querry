mod imp;

use glib::Object;
use gtk::{
    glib::{self, clone, subclass::types::ObjectSubclassIsExt},
    prelude::{EditableExt, WidgetExt},
};

use crate::database::get_database;
use crate::utils::{
    crud::{
        collections::get_single_collection,
        requests::{get_single_request, update_request_item, HTTPMethods},
    },
    messaging::{AppEvent, EVENT_CHANNEL},
    tokio_runtime::runtime,
};

glib::wrapper! {
    pub struct RequestsView(ObjectSubclass<imp::RequestsView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RequestsView {
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Update the http_method of request item.
    pub async fn update_request_method(self, http_method: HTTPMethods) {
        let collection_id = self.imp().collection_id.borrow().to_string();
        let request_id = self.imp().request_id.borrow().to_string();
        let requests_menu = self.imp().requests_menu.clone();

        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };

        match update_request_item(
            &request_id,
            None,
            None,
            Some(http_method.clone()),
            None,
            &db,
        ) {
            Ok(_) => {
                requests_menu.set_label(&http_method.to_string());
                // Dispatch new http_method to collection_row.
                runtime().spawn(async move {
                    EVENT_CHANNEL
                        .0
                        .send(AppEvent::UpdateHttpMethod(
                            http_method,
                            request_id,
                            collection_id,
                        ))
                        .expect("Channel should be open");
                });
            }
            Err(_) => {
                tracing::error!("Could not update http_method.");
            }
        };
    }

    /// Set children of RequestView to either visible or invisible.
    pub fn set_child_widgets_visibilty(&self, visibility: bool) {
        let actions_box = self.imp().actions_box.clone();
        let names_box = self.imp().names_box.clone();
        let separator = self.imp().separator.clone();
        let separator_2 = self.imp().separator_2.clone();

        actions_box.set_visible(visibility);
        names_box.set_visible(visibility);
        separator.set_visible(visibility);
        separator_2.set_visible(visibility);
    }

    /// Monitor changes to request_name
    pub fn montitor_request_name_changes(&self) {
        let request_name = self.imp().request_name.clone();
        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };

        request_name.connect_editing_notify(clone!(@weak self as this => move |item| {
            let request_id = this.imp().request_id.borrow().to_string();
            let collection_id = this.imp().collection_id.borrow().to_string();
            let new_text = item.text();

            match update_request_item(
                &request_id, Some(new_text.to_string()), None, None, None, &db
            ) {
                Ok(_) => {
                    // Dispatch new request name to the collectionsrow Widget.
                    runtime().spawn(async move {
                        EVENT_CHANNEL
                            .0
                            .send(AppEvent::RenameRequestItem(new_text.to_string(), request_id, collection_id))
                            .expect("Channel should be open");
                    });
                },
                Err(_) => {
                    tracing::error!("Could not update request name.");
                }
            };
        }));
    }

    /// Listen to messages to displa request.
    pub fn listen_request_view(&self) {
        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };

        glib::spawn_future_local(clone!(@weak self as this => async move {
            let mut rx = EVENT_CHANNEL.0.subscribe();

            while let Ok(response) = rx.recv().await {
                match response {
                    AppEvent::ViewRequestItem(request_id) => {
                        let request_item = match get_single_request(request_id, &db) {
                            Ok(data) => data,
                            Err(_) => {
                                tracing::error!("Could not get request item.");
                                return;
                            }
                        };

                        let collection_item = match get_single_collection(request_item.collection_id, &db) {
                            Ok(data) => data,
                            Err(_) => {
                                tracing::error!("Could not get collection item.");
                                return;
                            }
                        };

                        this.imp().collection_name.set_label(&collection_item.name);
                        this.imp().request_name.set_text(&request_item.name);
                        *this.imp().request_id.borrow_mut() = request_item.id;
                        *this.imp().collection_id.borrow_mut() = collection_item.id;
                        this.imp().requests_menu.set_label(&request_item.http_method.unwrap_or_default());
                        this.imp().entry_box.set_text(&request_item.url.unwrap_or_default());
                        this.set_child_widgets_visibilty(true);

                    }
                    _ => {},
                }
            }
        }));
    }

    /// Listen for changes in URL values and update item url.
    pub fn monitor_url_changes(&self) {
        let entry = self.imp().entry_box.clone();
        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };

        entry.connect_changed(clone!(@weak self as this => move |item| {
            let request_id = this.imp().request_id.borrow().to_string();
            match update_request_item(
                &request_id,
                None,
                None,
                None,
                Some(item.text().to_string()),
                &db,
            ) {
                Ok(_) => {
                }
                Err(_) => {
                    tracing::error!("Could not update url.");
                }
            };
        }));
    }
}
