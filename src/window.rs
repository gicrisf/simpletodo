use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, Entry, ListView};
use once_cell::sync::OnceCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/gicrisf/SimpleTodo/window.ui")]
    pub struct SimpletodoWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub entry: TemplateChild<Entry>,
        #[template_child]
        pub list_view: TemplateChild<ListView>,
        pub model: OnceCell<gio::ListStore>,
    }

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

    impl ObjectImpl for SimpletodoWindow {}
    impl WidgetImpl for SimpletodoWindow {}
    impl WindowImpl for SimpletodoWindow {}
    impl ApplicationWindowImpl for SimpletodoWindow {}
}

glib::wrapper! {
    pub struct SimpletodoWindow(ObjectSubclass<imp::SimpletodoWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl SimpletodoWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create SimpletodoWindow")
    }
}
