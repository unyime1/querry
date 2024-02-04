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
        requests::{get_single_request, update_request_item},
    },
    messaging::{AppEvent, EVENT_CHANNEL},
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

    /// Set children of RequestView to either visible or invisible.
    pub fn set_child_widgets_visibilty(&self, visibity: bool) {
        let collection_name = self.imp().collection_name.clone();
        let request_name = self.imp().request_name.clone();
        let divider = self.imp().divider.clone();

        collection_name.set_visible(visibity);
        request_name.set_visible(visibity);
        divider.set_visible(visibity);
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
            let new_text = item.text();

            match update_request_item(
                request_id, Some(new_text.to_string()), None, None, None, &db
            ) {
                Ok(_) => {
                    // Dispatch new request name to collection list.
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
            while let Ok(response) = EVENT_CHANNEL.1.recv().await {
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

                        this.set_child_widgets_visibilty(true);

                    }
                    _ => {},
                }
            }
        }));
    }
}
