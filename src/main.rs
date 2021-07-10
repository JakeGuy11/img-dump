use gtk::prelude::*;
use gtk::Align::{Start, End, Fill};
use gtk::{Application, ApplicationWindow, Box, Entry, Button, Label};

fn main()
{
    // Create some (sort of) globals that we'll need to use/access
    let mut target_path = std::env::current_dir().expect("Failed to get current directory!").as_path();

    // First parse all the cli args
    let args: Vec<String> = std::env::args().skip(1).collect();

    // If there are no args, assume they wanna keep using the current dir
    if args.len() != 0
    {
        if args[0].as_str() == "--help" || args[0].as_str() == "-h" { println! ("Help message will go here!"); }
    }

    let application = Application::new(Some("com.github.jakeguy11.img-dump"), gio::ApplicationFlags::FLAGS_NONE);

    application.connect_activate(|app|
    {
        let mut window = ApplicationWindow::new(app);
        window.set_title("img-dump");
        window = ApplicationWindow::builder().resizable(false).build();

        // Here we will define all the widgets and set their properties

        // Define the base container
        let base_container = Box::new(gtk::Orientation::Vertical, 10);
        base_container.set_valign(Fill);
        base_container.set_halign(Fill);
        base_container.set_margin(10);

        // Define the containers for all the things the user will interact with
        let entry_container = Box::new(gtk::Orientation::Horizontal, 10);
        entry_container.set_valign(Start);
        entry_container.set_halign(Fill);

        // Define the container for the actual image
        let image_container = Box::new(gtk::Orientation::Vertical, 10);
        image_container.set_valign(Fill);
        image_container.set_halign(Fill);       

        // Define the container for the exit button
        let exit_container = Box::new(gtk::Orientation::Horizontal, 10);
        exit_container.set_valign(End);
        exit_container.set_halign(End);

        // Define the container that will hold the `accept` and `open folder` buttons
        let user_buttons_container = Box::new(gtk::Orientation::Horizontal, 10);
        user_buttons_container.set_valign(End);
        user_buttons_container.set_halign(Fill);

        // Define a label for testing purposes
        let label = Label::new(Some("This is text"));
        
        // Define the Entry where the user will type their paths
        let user_entry = Entry::new();
        user_entry.set_valign(Fill);
        user_entry.set_halign(Fill);

        // Define the Accept button
        let accept_button = Button::with_label("Ok");
        accept_button.set_valign(Fill);
        accept_button.set_halign(Fill);

        // Define the Open Directory button
        let open_dir_button = Button::with_label("..");
        open_dir_button.set_valign(Fill);
        open_dir_button.set_halign(Fill);

        // Define the image that will display the current item
        let display_image = gtk::Image::from_file(std::path::Path::new("/home/jake/downloads/polka.png")); // This is just a test image - it will be updated eventually to each file in the current dir
        display_image.set_valign(Fill);
        display_image.set_halign(Fill);

        // Define the exit button
        let exit_button = Button::with_label("Exit");
        exit_button.set_valign(Fill);
        exit_button.set_halign(End);

        // Now we add all the widgets to boxes
        
        // Add the two buttons to their container
        user_buttons_container.add(&accept_button);
        user_buttons_container.add(&open_dir_button);
        
        // Add the Entry and the button box to the entry container
        entry_container.pack_start(&user_entry, true, true, 10);
        entry_container.add(&user_buttons_container);

        // Here is where we will eventually add the image to the image box
        image_container.add(&display_image);

        // Add the exit button to the exit container
        exit_container.add(&exit_button);

        // Now add all the boxes to their base elements
        base_container.add(&label);
        base_container.add(&entry_container);
        base_container.add(&image_container);
        base_container.add(&exit_container);
        
        window.add(&base_container);

        user_entry.connect_activate(move |entry_field|
        {
            let entered_text = &entry_field.text();
            &label.set_label(entered_text.as_str());
        });

        exit_button.connect_clicked(|_|
        {
            std::process::exit(0);
        });

        window.show_all();
    });
    
    application.run_with_args::<String>(&[]);
}
