mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SnippetObject(ObjectSubclass<imp::SnippetObject>);
}

impl SnippetObject {
    pub fn new(path: String, title: String) -> Self {
        Object::new(&[ ("path", &path), ("title", &title)])
            .expect("Failed to create `SnippetObject`.")
    }
}

#[derive(Default)]
pub struct SnippetData {
    pub path: String,
    pub title: String,
}
