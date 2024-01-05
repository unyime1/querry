mod collection_item;
mod imp;

use glib::Object;
use gtk::{
    gio::ListStore,
    glib,
    glib::{subclass::types::ObjectSubclassIsExt, Cast, CastNone},
    prelude::{BoxExt, GObjectPropertyExpressionExt, ListItemExt, ListModelExt, WidgetExt},
    Box, Image, Label, ListItem, ListView, SignalListItemFactory, SingleSelection, Widget,
};

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

    pub fn add_new_collection(&self) {}

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
        factory.connect_setup(move |_, list_item| {
            let box_widget = Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .hexpand(true)
                .visible(true)
                .css_classes(vec!["collection_box"])
                .build();

            let image_widget = Image::new();

            let label_widget = Label::new(None);
            label_widget.set_hexpand(true);

            let view_more_widget = Image::builder().icon_name("view-more-symbolic").build();

            box_widget.append(&image_widget);
            box_widget.append(&label_widget);
            box_widget.append(&view_more_widget);

            let list_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem");
            list_item.set_child(Some(&box_widget));

            list_item
                .property_expression("item")
                .chain_property::<CollectionItem>("name")
                .bind(&label_widget, "label", Widget::NONE);

            list_item
                .property_expression("item")
                .chain_property::<CollectionItem>("icon")
                .bind(&image_widget, "icon_name", Widget::NONE);
        });

        let selection_model = SingleSelection::new(Some(self.get_collections_store()));
        self.get_collections_list()
            .set_model(Some(&selection_model));
        self.get_collections_list().set_factory(Some(&factory));
        self.get_collections_list()
            .set_css_classes(&vec!["collections_list"]);
        self.get_collections_list().set_show_separators(true);
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
        let collections_list = self.get_collections_list();
        let collections_store = self.get_collections_store();

        if collections_store.n_items() > 0 {
            self.remove(&empty_collections_box);
        } else {
            self.remove(&collections_list);
        }
    }
}
