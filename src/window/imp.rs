use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, Entry, ListView};
use once_cell::sync::OnceCell;

// Object holding the state
#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/github/gicrisf/SimpleTodo/window/window.ui")]
pub struct SimpletodoWindow {
    // Template widgets
    #[template_child]
    pub header_bar: TemplateChild<gtk::HeaderBar>,
    // Hello world label
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    // We store references to the entry
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub list_view: TemplateChild<ListView>,
    pub model: OnceCell<gio::ListStore>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SimpletodoWindow {
    const NAME: &'static str = "SimpletodoWindow";
    type Type = super::SimpletodoWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObject
impl ObjectImpl for SimpletodoWindow {
    // &Self::Type = SimpletodoWindow
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        // AKA we build this window when the parent is constructed
        self.parent_constructed(obj);

        // Setup
        obj.setup_model();
        obj.setup_callbacks();
        obj.setup_factory();
    }  // constructed
}

// Trait shared by all widgets
impl WidgetImpl for SimpletodoWindow {}

// Trait shared by all windows
impl WindowImpl for SimpletodoWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for SimpletodoWindow {}
