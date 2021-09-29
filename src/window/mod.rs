mod imp;

use crate::todo_object::TodoObject;
use crate::snippet_row::SnippetRow;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, NoSelection, SignalListItemFactory};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    fn model(&self) -> &gio::ListStore {
        // Get state
        let imp = imp::Window::from_instance(self);
        imp.model.get().expect("Could not get model")
    }

    fn setup_model(&self) {
        // Create new model
        let model = gio::ListStore::new(TodoObject::static_type());

        // Get state and set model
        let imp = imp::Window::from_instance(self);
        imp.model.set(model).expect("Could not set model");

        // Add some items to the model.
        self.model().append(&TodoObject::new(false,"Testing".to_string()));

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.model()));
        imp.list_view.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        // Get state
        let imp = imp::Window::from_instance(self);
        let model = self.model();

        // Setup callback so that activation
        // creates a new todo object and clears the entry
        imp.entry
            .connect_activate(clone!(@weak model => move |entry| {
                let buffer = entry.buffer();
                let content = buffer.text();
                let todo_object = TodoObject::new(false, content);
                model.append(&todo_object);
                buffer.set_text("");
            }));
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `SnippetRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TodoRow`
            let snippet_row = SnippetRow::new();
            list_item.set_child(Some(&snippet_row));
        });

        // Tell factory how to bind `TodoRow` to a `TodoObject`
        factory.connect_bind(move |_, list_item| {
            // Get `TodoObject` from `ListItem`
            let todo_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<TodoObject>()
                .expect("The item has to be an `TodoObject`.");

            // Get `SnippetRow` from `ListItem`
            let snippet_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<SnippetRow>()
                .expect("The child has to be a `SnippetRow`.");

            snippet_row.bind(&todo_object);
        });

        // Tell factory how to unbind `SnippetRow` from `TodoObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `SnippetRow` from `ListItem`
            let snippet_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<SnippetRow>()
                .expect("The child has to be a `SnippetRow`.");

            snippet_row.unbind();
        });

        // Set the factory of the list view
        let imp = imp::Window::from_instance(self);
        imp.list_view.set_factory(Some(&factory));
    }
}
