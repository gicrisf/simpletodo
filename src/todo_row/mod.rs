mod imp;

use crate::todo_object::TodoObject;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

glib::wrapper! {
    pub struct TodoRow(ObjectSubclass<imp::TodoRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for TodoRow {
    fn default() -> Self {
        Self::new()
    }  // default ()
}

impl TodoRow {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `TodoRow`")
    }  // new

    pub fn bind(&self, todo_object: &TodoObject) {
        // Get state
        let imp = self.imp();
        let completed_button = imp.completed_button.get();
        let content_label = imp.content_label.get();
        let mut bindings = imp.bindings.borrow_mut();

        // Bind `todo_object.completed` to `todo_row.completed_button.active`
        let completed_button_binding = todo_object
            .bind_property("completed")
    }
}
