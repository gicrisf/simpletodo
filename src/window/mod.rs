mod imp;

// use gtk::prelude::*;
// use gtk::subclass::prelude::*;
use gtk::{gio, glib};

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
