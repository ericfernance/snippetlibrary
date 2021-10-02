use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label};
use std::cell::RefCell;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "snippet_row.ui")]
pub struct SnippetRow {
    #[template_child]
    pub path_label: TemplateChild<Label>,
    #[template_child]
    pub title_label: TemplateChild<Label>,
    // Vector holding the bindings to properties of `TodoObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SnippetRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SnippetRow";
    type Type = super::SnippetRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SnippetRow {}

// Trait shared by all widgets
impl WidgetImpl for SnippetRow {}

// Trait shared by all boxes
impl BoxImpl for SnippetRow {}
