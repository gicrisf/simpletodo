use glib::{ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

use super::TodoData;

// Object holding the state
#[derive(Default)]
pub struct TodoObject {
    pub data: Rc<RefCell<TodoData>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TodoObject {
    const NAME: &'static str = "TodoObject";
    type Type = super::TodoObject;
}
