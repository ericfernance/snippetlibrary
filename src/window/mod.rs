use glib::{clone, Object};
use gtk::{gio, glib, SingleSelection};
use gtk::{Application, NoSelection, SignalListItemFactory};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use snippetlibrary::*;

use crate::snippet_object::SnippetObject;
use crate::snippet_row::SnippetRow;
use sourceview5::prelude::{BufferExt, ViewExt};
use sourceview5::LanguageManager;

mod imp;

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
        let model = gio::ListStore::new(SnippetObject::static_type());

        // Get state and set model
        let imp = imp::Window::from_instance(self);
        imp.model.set(model).expect("Could not set model");

        // Wrap model with selection and pass it to the list view
        let selection_model = SingleSelection::new(Some(self.model()));
        imp.list_view.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        // Get state
        let imp = imp::Window::from_instance(self);
        let model = self.model();

        imp.list_view.model().unwrap().connect_selection_changed(|sel_model, position, items| {
            println!("Selection changed");
            let item = sel_model.item(position).unwrap().downcast::<SnippetObject>().expect("Nope");
            println!("{:?}", item.property("path").unwrap());
            println!("{:?}", item.property("title").unwrap());
        });
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

        // Tell factory how to bind `TodoRow` to a `SnippetObject`
        factory.connect_bind(move |_, list_item| {
            // Get `SnippetObject` from `ListItem`
            let snippet_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<SnippetObject>()
                .expect("The item has to be an `SnippetObject`.");

            // Get `SnippetRow` from `ListItem`
            let snippet_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<SnippetRow>()
                .expect("The child has to be a `SnippetRow`.");

            snippet_row.bind(&snippet_object);
        });

        // Tell factory how to unbind `SnippetRow` from `SnippetObject`
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

    pub fn load_data(&self) {
        let model = self.model();
        let snippets = snippetlibrary::SnippetCollection::new("/home/eric/Desktop/testingsnippets".to_string());
        for snippet in snippets.snippets() {
            println!("{:?}", snippet);
            model.append(&SnippetObject::new(snippet.path().to_string(), snippet.title().to_string()));
        }

        println!("{:?}", model);
    }

    pub fn setup_sourceview(&self) {
        println!("setting up the sourceview");
        let imp = imp::Window::from_instance(self);

        let buffer = sourceview5::Buffer::new(None);
        buffer.set_highlight_syntax(true);
        if let Some(ref language) = sourceview5::LanguageManager::new().language("rust") {
            buffer.set_language(Some(language));
        }
        /*if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("solarized-light") {
            buffer.set_style_scheme(Some(scheme));
        }*/
        buffer.set_text("");


        let view = sourceview5::View::with_buffer(&buffer);
        view.set_monospace(true);
        //view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
        view.set_show_line_numbers(true);
        view.set_highlight_current_line(true);
        view.set_tab_width(4);
        view.set_hexpand(true);
        imp.sourceview_window.set_child(Some(&view));
    }
}
