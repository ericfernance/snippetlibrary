#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;

use fs::File;
use std::{fs, io::prelude::*, path::PathBuf, sync::{Arc, RwLock}, thread, };
use gio::prelude::*;
//use sourceview::{Language, LanguageManagerExt};
use gtk::{prelude::*, Application, Window};


fn main() {
    glib::set_application_name("SnippetLibrary");

    let application = Application::new(Some("com.thisisericrobert.snippetlibrary"), Default::default());


    application.connect_activate(move |application| {
        // resources.gresources is created by build.rs
        // it includes all the files in the resources directory
        let resource_bytes = include_bytes!("./views/resources.gresource");
        let resource_data = glib::Bytes::from(&resource_bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);


        /*APP_BUILDER = Some(Builder::from_resource("/com/thisisericrobert/coderstoolbox/main.ui"));

        let builder = get_app_builder();*/
        //builder.add_from_file("ui/main.ui");
        //let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");

        //window.set_application(Some(application));
        //window.show_all();
    });

    application.run();
}
