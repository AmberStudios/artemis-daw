//deps

use std::fs;

fn main() {
    //init();
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

//so, the goal is to get file i/o

//mod stands for module, which is basically a class. adding pub at the beginning makes it global
pub mod file {
    use std::fs;
    use std::io;

    pub fn read() {
        //here is some nice copypasta straight from the docs

        let filename = "etc/layout.conf";

        println!("In file {}", filename);

        let contents = fs::read_to_string(filename)
            //throw error if file not found/not readable
            .expect("[artemis file i/o] Error while reading file");

        println!("With text:\n{}", contents);
    }
    pub fn write() -> std::io::Result<()> {
        let filename = "logs/test.log";

        fs::write(filename, "file write complete")?;
        Ok(())
    }

    pub fn remove() -> std::io::Result<()> {
        fs::remove_file("a.txt")?;
        Ok(())
    }
}

fn init() {
    file::remove();
    file::read();
    file::write();
    init_window();
}
fn init_window() {

}
