mod keygen;
mod queue;
mod progress_dialog;
mod downloader;
mod utils;
mod ticket;
mod game_list;
mod grid_cell;

use gtk::prelude::*;
use gtk::{Application, glib};

use game_list::game_list;
use wiiu_downloader_rs::{getTitleEntries, TITLE_CATEGORY_TITLE_CATEGORY_ALL};

fn main() -> glib::ExitCode {
    let title_entries = unsafe { getTitleEntries(TITLE_CATEGORY_TITLE_CATEGORY_ALL) };
    let app = Application::new(
        Some("com.xpl0itu.wiiu_downloader-rs"),
        Default::default(),
    );
    app.connect_activate(move |application| {game_list(&application, title_entries)});
    app.run()
}
