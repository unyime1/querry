mod imp;

use glib::Object;
use gtk::{
    gio,
    glib::{self, clone, subclass::types::ObjectSubclassIsExt, ObjectExt},
    prelude::{GtkWindowExt, WidgetExt},
    Application,
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    pub fn set_sizes(&self) {
        // Connect sidebar height
        let sidebar = self.imp().stack_sidebar.clone();

        self.connect_default_height_notify(clone!(@weak self as window => move |_| {
            let height = window.allocated_height();
            let new_stack_sidebar_height = (height as f32 * 0.9) as i32;

            sidebar.set_property("height-request", new_stack_sidebar_height);
        }));
    }
}
