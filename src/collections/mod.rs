mod collection_item;
mod collection_row;
mod imp;

use adw::{prelude::*, MessageDialog, ResponseAppearance};
use glib::Object;
use gtk::{
    gio::ListStore,
    glib,
    glib::{clone, subclass::types::ObjectSubclassIsExt, Cast, CastNone},
    Entry, ListItem, ListView, SignalListItemFactory, SingleSelection,
};

use crate::utils::{
    collections::{create_collection, get_all_collections, CollectionData},
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
        let collections_vec = match get_all_collections() {
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

    async fn new_collection(&self) {
        let window: Window = self.root().unwrap().downcast::<Window>().unwrap();

        // Create entry
        let entry = Entry::builder()
            .placeholder_text("Name")
            .activates_default(true)
            .build();

        let cancel_response = "cancel";
        let create_response = "create";

        // Create new dialog
        let dialog = MessageDialog::builder()
            .heading("New Collection")
            .modal(true)
            .transient_for(&window)
            .destroy_with_parent(true)
            .close_response(cancel_response)
            .default_response(create_response)
            .extra_child(&entry)
            .build();
        dialog.add_responses(&[(cancel_response, "Cancel"), (create_response, "Create")]);
        // Make the dialog button insensitive initially
        dialog.set_response_enabled(create_response, false);
        dialog.set_response_appearance(create_response, ResponseAppearance::Suggested);

        // Set entry's css class to "error", when there is no text in it
        entry.connect_changed(clone!(@weak dialog => move |entry| {
            let text = entry.text();
            let empty = text.is_empty();

            dialog.set_response_enabled(create_response, !empty);

            if empty {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
        }));

        let response = dialog.choose_future().await;

        // Return if the user chose `cancel_response`
        if response == cancel_response {
            return;
        }

        let name = entry.text().to_string();
        let collection_data = match create_collection(name) {
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
        self.get_collections_store().append(&collection_item);
        self.calc_visible_child();
    }

    pub fn listen_collection_delete(&self) {
        // The main loop executes the asynchronous block
        glib::spawn_future_local(async move {
            while let Ok(response) = EVENT_CHANNEL.1.recv().await {
                match response {
                    AppEvent::CollectionDeleted(data) => {
                        println!("Collection deleted: {}", data)
                    }
                    AppEvent::RequestDeleted(_) => todo!(),
                }
            }
        });
    }
}
