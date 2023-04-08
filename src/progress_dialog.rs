use crate::queue::set_queue_cancelled;
use crate::downloader::download_title;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, ProgressBar, Label};

pub fn progress_dialog() {
    let app = Application::new(
        Some("com.example.progress-dialog"),
        Default::default(),
    );
    
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Download Progress"));
        window.set_default_size(400, 100);

        let box_container = Box::new(gtk::Orientation::Vertical, 10);

        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(0.0);
        box_container.append(&progress_bar);

        let label = Label::new(Some("0%"));
        box_container.append(&label);

        let button = Button::builder()
            .label("Download")
            .build();
        let cancel_button = Button::builder()
            .label("Cancel")
            .sensitive(false)
            .build();
        let progress_bar_clone = progress_bar.clone();
        let label_clone = label.clone();
        let cancel_button_clone = cancel_button.clone();
        button.connect_clicked(move |_| {
            cancel_button_clone.set_sensitive(true);
            download_title("00050000101c9500", "BOTW EUR", &progress_bar_clone, &label_clone, &cancel_button_clone).unwrap();
            set_queue_cancelled(false);
        });
        box_container.append(&button);
        box_container.append(&cancel_button);

        window.set_child(Some(&box_container));
        window.show();
    });

    app.run();
}