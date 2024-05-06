mod collection_item;
mod collection_row;
mod imp;
mod requests;

use adw::prelude::*;
use glib::Object;
use gtk::{
    gio::ListStore,
    glib,
    glib::{clone, subclass::types::ObjectSubclassIsExt, Cast, CastNone},
    ListItem, ListView, SignalListItemFactory, SingleSelection,
};

use crate::database::get_database;
use crate::utils::{
    crud::collections::{
        create_collection, delete_collection, get_all_collections, CollectionData,
    },
    messaging::{AppEvent, EVENT_CHANNEL},
};
use crate::window::Window;
use collection_item::CollectionItem;
use collection_row::CollectionRow;

glib::wrapper! {
    pub struct CollectionsWindow(ObjectSubclass<imp::CollectionsWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CollectionsWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setup_collections(&self) {
        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };

        let collections_vec = match get_all_collections(&db) {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Error occured while getting collections.");
                Vec::new()
            }
        };

        self.bind_collections_model(collections_vec);
    }

    pub fn get_collections_store(&self) -> ListStore {
        self.imp()
            .collections_store
            .get()
            .expect("`collections` should be set in `setup_collections`.")
            .clone()
    }

    pub fn get_collections_list(&self) -> ListView {
        self.imp().collections_list.clone()
    }

    pub fn bind_collections_model(&self, collections_vec: Vec<CollectionData>) {
        let collection_model = ListStore::new::<CollectionItem>();
        self.imp()
            .collections_store
            .set(collection_model.clone())
            .expect("Could not set collections");

        let collections: Vec<CollectionItem> = collections_vec
            .into_iter()
            .map(|c| CollectionItem::new(&c.name, &c.id, "folder-visiting-symbolic"))
            .collect();
        self.get_collections_store().extend_from_slice(&collections);

        let factory = SignalListItemFactory::new();

        // Create an empty `CollectionRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `CollectionRow`
            let collection_row = CollectionRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&collection_row));
        });

        // Tell factory how to bind `CollectionRow` to a `CollectionItem`
        factory.connect_bind(move |_, list_item| {
            // Get `CollectionItem` from `ListItem`
            let collection_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<CollectionItem>()
                .expect("The item has to be an `CollectionItem`.");

            // Get `CollectionRow` from `ListItem`
            let collection_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<CollectionRow>()
                .expect("The child has to be a `CollectionRow`.");

            collection_row.bind(&collection_item);
        });

        // Tell factory how to unbind `CollectionRow` from `CollectionItem`
        factory.connect_unbind(move |_, list_item| {
            // Get `CollectionRow` from `ListItem`
            let collection_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<CollectionRow>()
                .expect("The child has to be a `CollectionRow`.");

            collection_row.unbind();
        });

        let selection_model = SingleSelection::new(Some(self.get_collections_store()));
        self.get_collections_list()
            .set_model(Some(&selection_model));
        self.get_collections_list().set_factory(Some(&factory));
        self.get_collections_list()
            .set_css_classes(&vec!["collections_list"]);
        self.get_collections_list().set_single_click_activate(true);
    }

    pub fn setup_collection_click(&self) {
        self.get_collections_list()
            .connect_activate(move |list_view, position| {
                let model = list_view.model().expect("The model has to exist.");
                let collection_item = model
                    .item(position)
                    .and_downcast::<CollectionItem>()
                    .expect("The item has to be a `CollectionItem`.");

                let openicon = collection_item.openicon();
                if openicon {
                    collection_item.update_icon("folder-visiting-symbolic");
                } else {
                    collection_item.update_icon("folder-drag-accept-symbolic");
                }
                collection_item.set_openicon(!openicon);
            });
    }

    pub fn calc_visible_child(&self) {
        let empty_collections_box = self.imp().empty_collections_box.clone();
        let collections_store = self.get_collections_store();
        let filled_collections_box = self.imp().filled_collections_box.clone();

        if collections_store.n_items() > 0 {
            empty_collections_box.set_visible(false);
            filled_collections_box.set_visible(true);
        } else {
            filled_collections_box.set_visible(false);
            empty_collections_box.set_visible(true);
        }
    }

    fn get_root_widget(&self) -> Window {
        let window: Window = self.root().unwrap().downcast::<Window>().unwrap();
        window
    }

    async fn new_collection(&self) {
        let name = String::from("New Collection");
        let db = match get_database() {
            Ok(data) => data,
            Err(_) => {
                tracing::error!("Could not get database connection.");
                return;
            }
        };
        let collection_data = match create_collection(name, &db) {
            Ok(data) => data,
            Err(error) => {
                tracing::error!("Could not create collection");
                println!("{}", error);
                return;
            }
        };

        let collection_item = CollectionItem::new(
            &collection_data.name,
            &collection_data.id,
            "folder-visiting-symbolic",
        );
        self.get_collections_store().insert(0, &collection_item);
        self.calc_visible_child();
    }

    /// Listen to delete connection signals and remove deleted collection.
    pub fn listen_collection_delete(&self) {
        glib::spawn_future_local(clone!(@weak self as this => async move {
            let collections_store = this.get_collections_store();
            let db = match get_database() {
                Ok(data) => data,
                Err(_) => {
                    tracing::error!("Could not get database connection.");
                    return
                }
            };

            let mut rx = EVENT_CHANNEL.0.subscribe();
            while let Ok(response) = rx.recv().await {
                match response {
                    AppEvent::CollectionDeleted(collection_id) => {
                        // Identify collection item from ListStore and remove.
                        let collection_item_index = collections_store
                            .iter::<CollectionItem>()
                            .position(|item| item.unwrap().id() == collection_id)
                            .map(|index| index as u32);

                        if let Some(index) = collection_item_index {
                            collections_store.remove(index);
                            delete_collection(collection_id, &db).expect("Can't delete item");
                            this.calc_visible_child();
                        }
                    },
                    _ => {},
                }
            }
        }));
    }
}
