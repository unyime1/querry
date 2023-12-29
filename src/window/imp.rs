use adw::{subclass::prelude::*, ApplicationWindow, Bin};
use glib::subclass::InitializingObject;
use gtk::{glib, Box, CompositeTemplate, StackSidebar};

// Initialize composite template for Window.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/etim/querry/window.ui")]
pub struct Window {
    #[template_child]
    pub stack_sidebar: TemplateChild<StackSidebar>,
    #[template_child]
    pub rest_box: TemplateChild<Box>,
    #[template_child]
    pub realtime_box: TemplateChild<Box>,
    #[template_child]
    pub settings_box: TemplateChild<Box>,
    #[template_child]
    pub rest_page: TemplateChild<Bin>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "QuerryWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all adwaita application windows
impl AdwApplicationWindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();

        let obj = self.obj();
        obj.connect_rest_clicked();
        obj.connect_realtime_clicked();
        obj.connect_settings_clicked();
        obj.fix_rest_ui();
        obj.set_active_sidebar_page();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        // Pass close request on to the parent
        self.parent_close_request()
    }
}
