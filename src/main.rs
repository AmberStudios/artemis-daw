//deps
use std::fs;

fn main() {
    init();
    gui::start();
}

pub mod gui {
    use druid::widget::{Button, Flex, Label};
    use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

    pub fn start() -> Result<(), PlatformError> {
        let main_window = WindowDesc::new(ui_builder);
        let data = 0_u32;
        AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
    }

    fn ui_builder() -> impl Widget<u32> {
        // The label text will be computed dynamically based on the current locale and count
        let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
        let label = Label::new(text).padding(5.0).center();
        let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

        Flex::column().with_child(label).with_child(button)
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
