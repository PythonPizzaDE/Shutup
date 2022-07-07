use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation, ButtonBox, ButtonBoxStyle};

use system_shutdown::{shutdown, logout, reboot};

fn main() {
    let app = Application::builder()
        .application_id("org.pythonpizza.Shutup")
        .build();
    
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .resizable(false)
            .default_width(1000)
            .default_height(300)
            .title("Shutup")
            .build();

        let button_box = ButtonBox::builder()
            .orientation(Orientation::Horizontal)
            .layout_style(ButtonBoxStyle::Expand)
            .build();

        let shutdown_button = Button::with_label("Shutdown");
        let restart_button = Button::with_label("Reboot");
        let logout_button = Button::with_label("Logout");

        button_box.add(&shutdown_button);
        button_box.add(&restart_button);
        button_box.add(&logout_button);

        shutdown_button.connect_clicked(|_| {
            shutdown().unwrap();
        });

        restart_button.connect_clicked(|_| {
            reboot().unwrap();
        });

        logout_button.connect_clicked(|_| {
            logout().unwrap();
        });

        window.add(&button_box);

        window.show_all();
    });

    app.run();
}
