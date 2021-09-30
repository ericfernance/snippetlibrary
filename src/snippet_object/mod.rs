mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SnippetObject(ObjectSubclass<imp::SnippetObject>);
}

impl SnippetObject {
    pub fn new(content: String) -> Self {
        Object::new(&[ ("content", &content)])
            .expect("Failed to create `SnippetObject`.")
    }
}

#[derive(Default)]
pub struct SnippetData {
    pub content: String,
}
