//deps

use std::fs;

fn main() {
    init();
    gui::app();
}

pub mod gui {
    extern crate gtk;
    extern crate gio;

    use gtk::prelude::*;
    use gio::prelude::*;

    use gtk::{Builder, Window};

    use std::path::Path;

    pub fn app() {

        gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

//I can't get it to load the theme


/*        let gui_theme = include_str!("../gui/theme/quartz-oneplus/gtk-3.0/gtk.css");

        let theme = gtk::CssProviderExt::load_from_file(gui_theme, &p);

        gtk::StyleContext::add_provider_for_screen(&gdk::auto::screen::Screen,theme,1);

*/

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
