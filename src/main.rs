use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Entry, Button, Label};

fn main()
{
    let application = Application::new(Some("com.github.jakeguy11.rust-app"), gio::ApplicationFlags::FLAGS_NONE);

    application.connect_activate(|app|
    {
        let window = ApplicationWindow::new(app);
        window.set_title("My actual window title");
        window.set_default_size(800, 600);

        let container = Box::new(gtk::Orientation::Vertical, 10);
        let label = Label::new(Some("This is text"));
        let input_text = Entry::new();
        let button = Button::with_label("I am a button");

        container.add(&label);
        container.add(&input_text);
        container.add(&button);
        window.add(&container);

        input_text.connect_activate(move |_|
        {
            let entered_text = &input_text.text();
            &label.set_label(entered_text.as_str());
        });

        window.show_all();
    });

    application.run();
}
