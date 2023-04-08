use wiiu_downloader_rs::{getTitleEntriesSize, getFormattedKind, getFormattedRegion, TITLE_CATEGORY_TITLE_CATEGORY_ALL};
use gtk::prelude::*;
use gtk::{gio, Application};


use crate::grid_cell::Entry;
use crate::grid_cell::GridCell;
use gtk::glib::BoxedAnyObject;

use std::cell::Ref;

use std::ffi::CStr;
use std::str;

struct Row {
    in_queue: bool,
    title_id: String,
    kind: String,
    region: String,
    name: String,
}

use wiiu_downloader_rs::TitleEntry;

pub fn game_list(app: &Application, game_titles: *const TitleEntry) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(716)
        .default_height(400)
        .application(app)
        .title("wiiu_downloader-rs")
        .build();

    let store = gio::ListStore::new(BoxedAnyObject::static_type());

    (0..unsafe { getTitleEntriesSize(TITLE_CATEGORY_TITLE_CATEGORY_ALL) }).for_each(|i| {
        let game_name: &CStr = unsafe { CStr::from_ptr((*game_titles.offset(i.try_into().unwrap())).name) };
        let game_name_str: &str = game_name.to_str().unwrap();

        let kind: &CStr = unsafe { CStr::from_ptr(getFormattedKind((*game_titles.offset(i.try_into().unwrap())).tid)) };
        let kind_str: &str = kind.to_str().unwrap();

        let region: &CStr = unsafe { CStr::from_ptr(getFormattedRegion((*game_titles.offset(i.try_into().unwrap())).region)) };
        let region_str: &str = region.to_str().unwrap();
        store.append(&BoxedAnyObject::new(Row {
            in_queue: false,
            title_id: "PLACEHOLDER".to_string(),
            kind: kind_str.to_string(),
            region: region_str.to_string(),
            name: game_name_str.to_string(),
        }))
    });
    let sel = gtk::SingleSelection::new(Some(store));
    let column_view = gtk::ColumnView::new(Some(sel));

    let in_queue_col_factory = gtk::SignalListItemFactory::new();
    let title_id_col_factory = gtk::SignalListItemFactory::new();
    let kind_col_factory = gtk::SignalListItemFactory::new();
    let region_col_factory = gtk::SignalListItemFactory::new();
    let name_col_factory = gtk::SignalListItemFactory::new();

    in_queue_col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::new();
        item.set_child(Some(&row));
    });

    in_queue_col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: if r.in_queue { "YES".to_string() } else { "NO".to_string() },
        };
        child.set_entry(&ent);
    });
    title_id_col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::new();
        item.set_child(Some(&row));
    });

    title_id_col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.title_id.to_string(),
        };
        child.set_entry(&ent);
    });

    kind_col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::new();
        item.set_child(Some(&row));
    });

    kind_col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.kind.to_string(),
        };
        child.set_entry(&ent);
    });

    region_col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::new();
        item.set_child(Some(&row));
    });

    region_col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.region.to_string(),
        };
        child.set_entry(&ent);
    });

    name_col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::new();
        item.set_child(Some(&row));
    });

    name_col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.name.to_string(),
        };
        child.set_entry(&ent);
    });

    let in_queue_col = gtk::ColumnViewColumn::new(Some("Queue"), Some(in_queue_col_factory));
    let title_id_col = gtk::ColumnViewColumn::new(Some("TitleID"), Some(title_id_col_factory));
    let kind_col = gtk::ColumnViewColumn::new(Some("Kind"), Some(kind_col_factory));
    let region_col = gtk::ColumnViewColumn::new(Some("Region"), Some(region_col_factory));
    let name_col = gtk::ColumnViewColumn::new(Some("Name"), Some(name_col_factory));

    column_view.set_hexpand(true);
    column_view.set_vexpand(true);
    column_view.append_column(&in_queue_col);
    column_view.append_column(&title_id_col);
    column_view.append_column(&kind_col);
    column_view.append_column(&region_col);
    column_view.append_column(&name_col);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .hexpand(true)
        .vexpand(true)
        .build();

    scrolled_window.set_child(Some(&column_view));

    window.set_child(Some(&scrolled_window));
    window.show();
}