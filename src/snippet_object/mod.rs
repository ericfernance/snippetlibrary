mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SnippetObject(ObjectSubclass<imp::SnippetObject>);
}

impl SnippetObject {
    pub fn new(content: String, title: String) -> Self {
        Object::new(&[ ("content", &content), ("title", &title)])
            .expect("Failed to create `SnippetObject`.")
    }
}

#[derive(Default)]
pub struct SnippetData {
    pub content: String,
    pub title: String,
}
