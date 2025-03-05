use gtk::prelude::*;
use gtk::{Button, Entry, TextView, Window, WindowType};
use std::process::Command;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Solara Executor");
    window.set_default_size(400, 300);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let script_entry = Entry::new();
    let execute_button = Button::with_label("Execute");
    let output_view = TextView::new();
    output_view.set_editable(false);

    vbox.pack_start(&script_entry, false, false, 0);
    vbox.pack_start(&execute_button, false, false, 0);
    vbox.pack_start(&output_view, true, true, 0);
    window.add(&vbox);

    execute_button.connect_clicked(move |_| {
        let script = script_entry.get_text().unwrap();
        let output = execute_script(&script);
        output_view.get_buffer().unwrap().set_text(&output);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn execute_script(script: &str) -> String {
    let output = Command::new("roblox_exe_path")
        .arg("-execute")
        .arg(script)
        .output()
        .expect("Failed to execute script");

    String::from_utf8_lossy(&output.stdout).to_string()
}