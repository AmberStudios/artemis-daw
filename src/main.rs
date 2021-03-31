//deps

use std::fs;

fn main() {
    init();
    gui::app();
}

pub mod gui {
    extern crate gdk;
    extern crate gio;
    //extern crate glib;
    extern crate gtk;

    use gtk::prelude::*;
    use gio::prelude::*;
    use gtk::{Builder, Window, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};

    use std::path::Path;

    pub fn app() {

        gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    let gui_theme = include_str!("../gui/theme/artemis-theme/gtk3.css");
//load the theme
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(gui_theme.as_bytes())
        .expect("Failed to load CSS");

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

        //load the glade file to use as gui
        let gui_src = include_str!("../gui/main.glade");

        let gui_path = Path::new("gui/main.glade");

        let builder = Builder::from_string(gui_src);

        //create the variable "window" with the type: "Window"
        let window: Window = builder.get_object("main_window").unwrap();

        //get close event
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(true)
        });

        window.show_all();
        gtk::main();
    }

}

//this loads the file "file.rs". Meaning that you can import modules from other files.
mod file;

fn init() {

    file::remove();
    file::read();
    file::write();
    init_window();
}
fn init_window() {

}
