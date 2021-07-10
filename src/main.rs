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

        // Define all the containers
        let entry_container = Box::new(gtk::Orientation::Horizontal, 10);
        entry_container.set_valign(gtk::Align::Start);

        let label = Label::new(Some("This is text"));
        let user_entry = Entry::new();
        user_entry.set_valign(gtk::Align::Fill);
        let exit_button = Button::with_label("Exit");
        exit_button.set_valign(gtk::Align::Fill);

        //base_container.add(&label);
        entry_container.pack_start(&user_entry, true, true, 10);
        entry_container.add(&exit_button);
        //base_container.add(&entry_container);

        //window.add(&label);
        window.add(&entry_container);

        user_entry.connect_activate(move |entry_field|
        {
            let entered_text = format! ("{}", &entry_field.alignment());
            &label.set_label(entered_text.as_str());
        });

        exit_button.connect_clicked(move |_|
        {
            std::process::exit(0);
        });

        window.show_all();
    });

    application.run();
}
