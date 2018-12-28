use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new_from_string(include_str!("./ui/PostgresAdminTool.glade"));

    let window: gtk::Window = builder.get_object("window-connections").unwrap();
    // let button: gtk::Button = builder.get_object("button1").unwrap();
    // let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

    // button.connect_clicked(move |_| {
    //     dialog.run();
    //     dialog.hide();
    // });

    let new_connection_name: gtk::Entry = builder.get_object("new_connection_name").unwrap();

    new_connection_name.connect_changed(|input| {
        println!("CHanged m8 {:?}", input.get_text());
    });

    window.show();

    // Handle closing of the window.
    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();

        // Let the default handler destroy the window.
        Inhibit(false)
    });

    gtk::main();
}
