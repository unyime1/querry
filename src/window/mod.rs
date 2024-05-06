mod imp;

use adw::prelude::*;
use glib::Object;
use gtk::{
    gio,
    glib::{self, clone, subclass::types::ObjectSubclassIsExt},
    Application,
};

use crate::rest::RestWindow;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    /// Create new window
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    /// Check window sizes.
    pub fn check_sizes(&self) {
        self.connect_default_height_notify(clone!(@weak self as window => move |_| {
            let height = window.allocated_height();
            let width = window.allocated_width();

            println!("{}, {}", width, height);
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

    pub fn fix_rest_ui(&self) {
        let rest_box = self.imp().rest_page.clone();
        let rest_window = RestWindow::new();
        rest_box.set_child(Some(&rest_window));
    }

    pub fn set_active_sidebar_page(&self) {
        let rest_box = self.imp().rest_box.clone();
        rest_box.add_css_class("active-sidebar");
    }
}
