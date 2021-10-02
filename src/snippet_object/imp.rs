use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

use super::SnippetData;

// Object holding the state
#[derive(Default)]
pub struct SnippetObject {
    pub data: Rc<RefCell<SnippetData>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SnippetObject {
    const NAME: &'static str = "SnippetObject";
    type Type = super::SnippetObject;
    type ParentType = glib::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for SnippetObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_string(
                    // Name
                    "path",
                    // Nickname
                    "path",
                    // Short description
                    "path",
                    // Default value
                    None,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_string("title","title","title",None, ParamFlags::READWRITE),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "path" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().path = input_value;
            }
            "title"=>{
                let input_value = value.get().expect("The value needs to be of type string");
                self.data.borrow_mut().title = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "path" => self.data.borrow().path.to_value(),
            "title"=>self.data.borrow().title.to_value(),
            _ => unimplemented!(),
        }
    }
}
