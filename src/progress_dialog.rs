use gtk::{prelude::*, Button, Label, Window, ProgressBar, Box};

pub struct Progress {
    pub progress: ProgressBar,
    pub label: Label,
}
unsafe impl Send for Progress {}
unsafe impl Sync for Progress {}

pub fn progress_dialog(progress: &Progress) -> Window {
    let cancel_button = Button::builder()
        .label("Cancel")
        .build();

    cancel_button.connect_clicked(|_| {
        println!("Cancel button clicked");
    });

    let main_box = Box::builder()
        .orientation(gtk::Orientation::__Unknown(gtk::ffi::GTK_ORIENTATION_VERTICAL))
        .spacing(5)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    main_box.append(&progress.label);
    main_box.append(&progress.progress);
    main_box.append(&cancel_button);

    let progress_window = Window::builder()
        .title("wiiu_downloader-rs")
        .modal(true)
        .child(&main_box)
        .build();

    return progress_window;
}