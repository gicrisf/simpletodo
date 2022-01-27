use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label};
use std::cell::RefCell;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/github/gicrisf/SimpleTodo/todo_row/todo_row.ui")]
pub struct TodoRow {
    #[template_child]
    pub completed_button: TemplateChild<CheckButton>,
    #[template_child]
    pub content_label: TemplateChild<Label>,
    // Vector holding the bindings to properties of `TodoObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TodoRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TodoRow";
    type Type = super::TodoRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass)
    } // class_init

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }  // instance_init
}

// Trait shared by all GObject
impl ObjectImpl for TodoRow {}

// Trait shared by all widgets
impl WidgetImpl for TodoRow {}

// Trait shared by all boxes
impl BoxImpl for TodoRow {}
