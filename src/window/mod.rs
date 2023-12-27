mod imp;

use glib::Object;
use gtk::{
    gio,
    glib::{self, clone, subclass::types::ObjectSubclassIsExt, ObjectExt},
    prelude::*,
    Application,
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    /// Create new window
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    /// Calculate height on resize.
    pub fn set_sizes(&self) {
        let sidebar = self.imp().stack_sidebar.clone();

        self.connect_default_height_notify(clone!(@weak self as window => move |_| {
            let height = window.allocated_height();
            let new_stack_sidebar_height = (height as f32 * 0.9) as i32;

            sidebar.set_property("height-request", new_stack_sidebar_height);
        }));
    }

    /// Connect clicked signal to rest button.
    pub fn connect_rest_clicked(&self) {
        let stack_sidebar = self.imp().stack_sidebar.clone();

        let rest_box = self.imp().rest_box.clone();
        let realtime_box = self.imp().realtime_box.clone();
        let settings_box = self.imp().settings_box.clone();
        let rest_box_clone = rest_box.clone();

        let gesture_rest = gtk::GestureClick::new();
        gesture_rest.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            let stack = &stack_sidebar.stack().expect("No stack");

            // Manage active classes.
            stack.set_visible_child_name("rest_page");
            rest_box_clone.add_css_class("active-sidebar");
            realtime_box.remove_css_class("active-sidebar");
            settings_box.remove_css_class("active-sidebar");
        });
        rest_box.add_controller(gesture_rest);
    }

    /// Connect clicked signal to realtime button.
    pub fn connect_realtime_clicked(&self) {
        let stack_sidebar = self.imp().stack_sidebar.clone();

        let rest_box = self.imp().rest_box.clone();
        let realtime_box = self.imp().realtime_box.clone();
        let settings_box = self.imp().settings_box.clone();
        let realtime_box_clone = realtime_box.clone();

        let gesture_realtime = gtk::GestureClick::new();
        gesture_realtime.connect_released(move |gesture_real, _, _, _| {
            gesture_real.set_state(gtk::EventSequenceState::Claimed);
            let stack = &stack_sidebar.stack().expect("No stack");

            // Manage active classes.
            stack.set_visible_child_name("realtime_page");
            realtime_box_clone.add_css_class("active-sidebar");
            settings_box.remove_css_class("active-sidebar");
            rest_box.remove_css_class("active-sidebar");
        });
        realtime_box.add_controller(gesture_realtime);
    }

    /// Connect clicked signal to settings button.
    pub fn connect_settings_clicked(&self) {
        let stack_sidebar = self.imp().stack_sidebar.clone();

        let rest_box = self.imp().rest_box.clone();
        let realtime_box = self.imp().realtime_box.clone();
        let settings_box = self.imp().settings_box.clone();
        let settings_box_clone = settings_box.clone();

        let gesture_settings = gtk::GestureClick::new();
        gesture_settings.connect_released(move |gesture_s, _, _, _| {
            gesture_s.set_state(gtk::EventSequenceState::Claimed);
            let stack = &stack_sidebar.stack().expect("No stack");

            // Manage active classes.
            stack.set_visible_child_name("settings_page");
            settings_box_clone.add_css_class("active-sidebar");
            rest_box.remove_css_class("active-sidebar");
            realtime_box.remove_css_class("active-sidebar");
        });
        settings_box.add_controller(gesture_settings);
    }
}
