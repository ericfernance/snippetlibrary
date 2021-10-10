mod snippet_object;
mod snippet_row;
mod window;

#[macro_use]
extern crate lazy_static;
extern crate copypasta;

use gtk::prelude::*;
use gtk::Application;
use window::Window;
use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    // Initialize logger
    //pretty_env_logger::init();

    // Create a new application
    let app = Application::builder()
        .application_id("com.thisisericrobert.snippetlibrary")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.show();
}

