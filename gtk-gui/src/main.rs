use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new();

    builder
        .add_from_file("src/ui/PostgresAdminTool.glade")
        .unwrap();

    let window: gtk::Window = builder.get_object("window-connections").unwrap();
    // let button: gtk::Button = builder.get_object("button1").unwrap();
    // let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

    // button.connect_clicked(move |_| {
    //     dialog.run();
    //     dialog.hide();
    // });

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
