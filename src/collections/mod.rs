mod collection_item;
mod imp;

use glib::Object;
use gtk::glib::{Cast, CastNone};
use gtk::{glib, SignalListItemFactory, ListItem, Widget, SingleSelection, ListView};
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::{BoxExt, ListItemExt, GObjectPropertyExpressionExt, ListModelExt};
use gtk::gio::ListStore;
use gtk::{Box, Image, Label};

use crate::utils::collections::{get_all_collections, CollectionData};
use collection_item::CollectionItem;

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

    pub fn add_new_collection(&self) {
    }

    pub fn bind_collections_model(&self, collections_vec: Vec<CollectionData>) {
        let collection_model = ListStore::new::<CollectionItem>();
        self.imp()
            .collections_store
            .set(collection_model.clone())
            .expect("Could not set collections");

        let collections: Vec<CollectionItem> = collections_vec
        .into_iter()
        .map(|c| CollectionItem::new(&c.name, &c.id))
        .collect();

        self.get_collections_store().extend_from_slice(&collections);
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            // Create label
            let label = Label::new(None);
            let list_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem");
            list_item.set_child(Some(&label));
    
            // Bind `list_item->item->number` to `label->label`
            list_item
                .property_expression("item")
                .chain_property::<CollectionItem>("name")
                .bind(&label, "label", Widget::NONE);
        });

        let selection_model = SingleSelection::new(Some(self.get_collections_store()));
        self.get_collections_list().set_model(Some(&selection_model));
        self.get_collections_list().set_factory(Some(&factory));

        self.get_collections_list().connect_activate(move |list_view, position| {
            // Get `IntegerObject` from model
            let model = list_view.model().expect("The model has to exist.");
            let collection_item = model
                .item(position)
                .and_downcast::<CollectionItem>()
                .expect("The item has to be an `IntegerObject`.");
    
            println!("{}", collection_item.name());
        });
    }

    pub fn bind_collections_list(&self, mut collections_vec: Vec<CollectionData>) {
        collections_vec.push(CollectionData {
            id: "hi".to_string(),
            name: "Test".to_string(),
        });

        // Convert `Vec<CollectionData>` to `Vec<CollectionItem>`
        let collections: Vec<CollectionItem> = collections_vec
            .into_iter()
            .map(|c| CollectionItem::new(&c.name, &c.id))
            .collect();

        let collection_list = self.imp().collections_list.clone();
        for collection_item in collections {
            let box_widget = Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .hexpand(true)
                .visible(true)
                .build();

            let image_widget = Image::builder()
                .icon_name("folder-drag-accept-symbolic")
                .build();

            box_widget.append(&image_widget);

            let label_widget = Label::builder().label(collection_item.name()).build();
            box_widget.append(&label_widget);

            // collection_list.append(&box_widget);
        }
    }
}
