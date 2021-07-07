extern crate cascade;

use gtk::prelude::*;
use std::process;

fn main() {
    // Set application names
    glib::set_program_name(Some("First GTK App"));
    glib::set_application_name("First GTK App");

    // Exit if init failed
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Run gtk's main function - will wait until app is exited
    gtk::main();
}
