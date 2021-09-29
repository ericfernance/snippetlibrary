mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SnippetObject(ObjectSubclass<imp::SnippetObject>);
}

impl SnippetObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
            .expect("Failed to create `SnippetObject`.")
    }
}

#[derive(Default)]
pub struct SnippetData {
    pub completed: bool,
    pub content: String,
}
