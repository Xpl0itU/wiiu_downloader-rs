use gtk::prelude::*;
use gtk::{Window, Box, Button, ProgressBar, Label};

pub fn progress_dialog() -> (Window, ProgressBar, Label, Button) {
    let window = Window::new();
    window.set_title(Some("Download Progress"));
    window.set_default_size(400, 100);

    let box_container = Box::new(gtk::Orientation::Vertical, 10);

    let progress_bar = ProgressBar::new();
    progress_bar.set_fraction(0.0);
    box_container.append(&progress_bar);

    let label = Label::new(Some("0%"));
    box_container.append(&label);

    let cancel_button = Button::builder()
        .label("Cancel")
        .sensitive(false)
        .build();

    box_container.append(&cancel_button);

    window.set_child(Some(&box_container));
    window.show();

    return (window, progress_bar, label, cancel_button);
}