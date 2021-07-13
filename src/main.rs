use gtk::prelude::*;
use gtk::Align::{Start, End, Fill};
use gtk::{Application, ApplicationWindow, Box, Entry, Button, Label};
use std::path::PathBuf;
use std::ffi::OsString;
use std::cell::RefCell;
use std::rc::Rc;

// An enum for "does not exist" vs "out of images"
enum ReasonForFail
{
    DoesNotExist,
    OutOfImages,
    NoPermission
}

fn populate_pathbuf_vec(target_vec: &mut Vec<PathBuf>, target_path: &PathBuf) -> Result<(), ()>
{
    // Clear the vector
    target_vec.clear();

    for current_item in std::fs::read_dir(target_path.as_path()).expect("Failed to read target dir!")
    {
        // Do some checks to make sure we want to add this entry
        if let Err(e) = current_item { eprintln! ("Failed to parse item in target dir: {}", e); continue; }

        // If it's not an error, we can unwrap it and do whatever we need to
        let item_path = current_item.unwrap().path();
        
        // Check if it's a directory or unsupported filetype
        if item_path.is_dir() { continue; }

        if item_path.extension() == Some(&OsString::from("jpg")) { target_vec.push(item_path); }
        else if item_path.extension() == Some(&OsString::from("jpeg")) { target_vec.push(item_path); }
        else if item_path.extension() == Some(&OsString::from("png")) { target_vec.push(item_path); }
        else if item_path.extension() == Some(&OsString::from("bmp")) { target_vec.push(item_path); }
        else if item_path.extension() == Some(&OsString::from("gif")) { target_vec.push(item_path); }
    }

    // If there are no files to go through, return an error
    if target_vec.is_empty() { Err(()) }
    else { Ok(()) }
}

fn update_image(target_image: &gtk::Image, image_vec_refcell: &RefCell<Vec<PathBuf>>, max_dimension: i32) -> Result<(), ReasonForFail>
{

    // Get the last item from the vector
    let wrapped_image_to_update = image_vec_refcell.borrow_mut().pop();
    
    // Check for potential problems - no more images, deleted images for now
    if let None = wrapped_image_to_update { return Err(ReasonForFail::OutOfImages); }
    let image_to_update = wrapped_image_to_update.unwrap();
    if !image_to_update.exists() { return Err(ReasonForFail::DoesNotExist); }
    
    // Create a PixBuf to normalize the image size
    let img_to_set = gdk_pixbuf::Pixbuf::from_file_at_scale(image_to_update.as_path(), max_dimension, max_dimension, true).expect("Failed to open file into PixBuf!");

    // We've checked for errors - now do what we need to with the image
    target_image.set_from_pixbuf(Some(&img_to_set));

    Ok(())
}

fn move_image(file_to_move: &PathBuf, target_path: &mut PathBuf) -> Result<(), ReasonForFail>
{
    // Make sure the file exists
    if !file_to_move.exists() { return Err(ReasonForFail::DoesNotExist); }

    // Clone the target PathBuf the target path
    let mut target_dir = target_path.clone();
    // Remove the file from the target
    target_dir.pop();

    // Get the extension and put it on the target
    let src_ext = file_to_move.extension().expect("Could not extract source extension!");
    target_path.set_extension(src_ext);

    // Create the target dir if it doesn't exist
    let create_dir_res = std::fs::create_dir_all(target_dir.as_path());
    if let Err(_) = create_dir_res { return Err(ReasonForFail::NoPermission); }

    // Copy the actual file
    let copy_res = std::fs::copy(&file_to_move, target_path);
    if let Err(_) = copy_res { return Err(ReasonForFail::NoPermission); }

    // Remove the original
    let remove_res = std::fs::remove_file(file_to_move);
    if let Err(_) = remove_res { return Err(ReasonForFail::NoPermission); }

    Ok(())
}

fn main()
{
    // Create some (sort of) globals that we'll need to use/access
    let mut target_path = std::env::current_dir().expect("Failed to get current directory!");
    let image_size = 600; // This will eventually become a cli arg

    // First parse all the cli args
    let args: Vec<String> = std::env::args().skip(1).collect();

    // If there are no args, assume they wanna keep using the current dir
    if args.len() != 0
    {
        if args[0].as_str() == "--help" || args[0].as_str() == "-h" { println! ("Help message will go here!"); }
        else
        {
            target_path = std::path::PathBuf::from(args[0].to_string());
        }
    }

    let application = Application::new(Some("com.github.jakeguy11.img-dump"), gio::ApplicationFlags::FLAGS_NONE);

    application.connect_activate(move |app|
    {
        // Create the vector of the images
        let images_vec = Rc::new(RefCell::new(Vec::new()));

        let populate_result = populate_pathbuf_vec(&mut images_vec.borrow_mut(), &target_path);
        if let Err(_) = populate_result { eprintln! ("There are no valid images in the targeted directory!"); std::process::exit(1); }
        println! ("{:?}", &images_vec);

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
        let display_image = gtk::Image::from_file(&target_path.as_path()); // This is just a test image - it will be updated eventually to each file in the current dir
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

        user_entry.connect_activate({
            let image_to_set = display_image.clone();
            let ownable_images_vec = Rc::clone(&images_vec);
            move |entry_field|
            {
                loop
                {
                    // Try to update the image
                    let update_result = update_image(&image_to_set, &ownable_images_vec, image_size);
                    
                    // Check what the result is and handle it
                    if let Err(ReasonForFail::OutOfImages) = update_result { eprintln! ("Out of images! Select a new directory!"); break; }
                    if let Err(ReasonForFail::DoesNotExist) = update_result { eprintln! ("Current image does not exist!"); }
                    if let Ok(_) = update_result { break; }
                }
            }
        });

        accept_button.connect_clicked({
            let image_to_set = display_image.clone();
            let ownable_images_vec = Rc::clone(&images_vec);
            move |_|
            {
                loop
                {
                    // Try to update the image
                    let update_result = update_image(&image_to_set, &ownable_images_vec, image_size);
                    
                    // Check what the result is and handle it
                    if let Err(ReasonForFail::OutOfImages) = update_result { eprintln! ("Out of images! Select a new directory!"); break; }
                    if let Err(ReasonForFail::DoesNotExist) = update_result { eprintln! ("Current image does not exist!"); }
                    if let Ok(_) = update_result { break; }
                }
            }
        });
        
        loop
        {
            // Try to update the image
            let update_result = update_image(&display_image, &images_vec, image_size);
            
            // Check what the result is and handle it
            if let Err(ReasonForFail::OutOfImages) = update_result { eprintln! ("Out of images! Select a new directory!"); break; }
            if let Err(ReasonForFail::DoesNotExist) = update_result { eprintln! ("Current image does not exist!"); }
            if let Ok(_) = update_result { break; }
        }

        exit_button.connect_clicked(|_|
        {
            std::process::exit(0);
        });

        window.show_all();
    });
    
    application.run_with_args::<String>(&[]);
}
