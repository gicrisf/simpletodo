mod imp;

use glib::Object;
use gtk::glib;

// Create a wrapper around the Object
glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}

// Implement the appropriate traits
impl TodoObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
            .expect("Failed to create `TodoObject`")
    }  // new
}

#[derive(Default)]
pub struct TodoData {
    pub completed: bool,
    pub content: String,
}
