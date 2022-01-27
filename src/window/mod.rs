mod imp;

use crate::todo_object::TodoObject;
use crate::todo_row::TodoRow;
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{NoSelection, SignalListItemFactory};

// Create a wrapper around the Object
glib::wrapper! {
    pub struct SimpletodoWindow(ObjectSubclass<imp::SimpletodoWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

// Implement the appropriate traits
impl SimpletodoWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        // Create new window
        glib::Object::new(&[("application", application)])
            .expect("Failed to create SimpletodoWindow")
    }

    fn model(&self) -> &gio::ListStore {
        // Get state and return model
        // let imp = imp::SimpletodoWindow::from_instance(self);
        let imp = self.imp();
        imp.model.get().expect("Could not get model")
    }  // model

    fn setup_model(&self) {
        // Create new model
        let model = gio::ListStore::new(TodoObject::static_type());

        // Get state
        // let imp = imp::SimpletodoWindow::from_instance(self);
        let imp = self.imp();
        imp.model.set(model).expect("Could not set model");

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.model()));
        imp.list_view.set_model(Some(&selection_model));
    }  // setup_model

    fn setup_callbacks(&self) {
        // Get state
        // let imp = imp::SimpletodoWindow::from_instance(self);
        let imp = self.imp();
        let model = self.model();

        // Setup callback so that activation
        // Creates a new todo object and clears the entry
        imp.entry
            .connect_activate(clone!(@weak model => move |entry| {
                let buffer = entry.buffer();
                let content = buffer.text();
                let todo_object = TodoObject::new(false, content);
                model.append(&todo_object);
                buffer.set_text("");
            }));
    }  // setup_callbacks

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TodoRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TodoRow`
            let todo_row = TodoRow::new();
            list_item.set_child(Some(&todo_row));
        });

        // Tell factory how to bind `TodoRow` to a `TodoObject`
        factory.connect_bind(move |_, list_item| {
            // Get `TodoObject` from `ListItem`
            let todo_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<TodoObject>()
                .expect("The item has to be a `TodoObject`.");

            // Get `TodoRow` from `ListItem`
            let todo_row = list_item
                .child()
                .expect("The child has to exists.")
                .downcast::<TodoRow>()
                .expect("The child has to be a `TodoRow`.");

            todo_row.bind(&todo_object);
        });

        // Tell factory how to unbind `TodoRow` from `TodoObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TodoRow` from `ListItem`
            let todo_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TodoRow>()
                .expect("The child has to be a `TodoRow`.");

            todo_row.unbind();
        });

        // Set the factory of the list view
        // let imp = imp::SimpletodoWindow::from_instance(self);
        let imp = self.imp();
        imp.list_view.set_factory(Some(&factory));
    }  // setup_factory
}
