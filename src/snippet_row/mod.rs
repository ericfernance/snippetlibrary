mod imp;

use crate::snippet_object::SnippetObject;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};

glib::wrapper! {
    pub struct SnippetRow(ObjectSubclass<imp::SnippetRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for SnippetRow {
    fn default() -> Self {
        Self::new()
    }
}

impl SnippetRow {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `SnippetRow`.")
    }

    pub fn bind(&self, snippet_object: &SnippetObject) {
        // Get state
        let imp = imp::SnippetRow::from_instance(self);
        let content_label = imp.content_label.get();
        let mut bindings = imp.bindings.borrow_mut();

        // Bind `snippet_object.content` to `snippet_row.content_label.label`
        let content_label_binding = snippet_object
            .bind_property("content", &content_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build()
            .expect("Could not bind properties");
        // Save binding
        bindings.push(content_label_binding);

    }

    pub fn unbind(&self) {
        // Get state
        let imp = imp::SnippetRow::from_instance(self);

        // Unbind all stored bindings
        for binding in imp.bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
