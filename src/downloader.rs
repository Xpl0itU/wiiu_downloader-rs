use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

use gtk::prelude::*;
use gtk::{Button, ProgressBar, Label, glib};

use crate::keygen::generate_key;
use crate::queue::{set_queue_cancelled, get_queue_cancelled};
use crate::utils::path_exists;
use crate::ticket::create_ticket;

pub fn download_file(url: &str, path: &str, progress_bar: &ProgressBar, label: &Label, cancel_button: &Button) -> Result<(), String> {
    // Reqwest setup
    let mut res = reqwest::blocking::Client::new()
        .get(url)
        .send()
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // download chunks
    let mut file = std::fs::File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut writer = Cursor::new(Vec::new());

    while !get_queue_cancelled() {
        let mut buffer = [0; 1024];
        let bytes_read = res.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        downloaded += bytes_read as u64;
        let progress = downloaded as f64 / total_size as f64;
        progress_bar.set_fraction(progress);

        let percentage = progress * 100.0;
        let percentage_string = format!("{:.0}%", percentage);
        label.set_text(&percentage_string);
        while glib::MainContext::pending(&glib::MainContext::default()) {
            glib::MainContext::iteration(&glib::MainContext::default(), true);
        }

        writer.write_all(&buffer[..bytes_read]).unwrap();

        cancel_button.connect_clicked(move |c_button| {
            set_queue_cancelled(true);
            c_button.set_sensitive(false);
        });
    }

    std::mem::drop(res);
    if get_queue_cancelled() {
        std::fs::remove_file(path).or(Err(format!("Failed to remove file '{}'", path)))?;
    } else {
        file.write_all(&writer.into_inner()).unwrap();
    }

    Ok(())
}

pub fn download_title(title_id: &str, name: &str, progress_bar: &ProgressBar, label: &Label, cancel_button: &Button) -> Result<(), String> {
    if !path_exists(name) {
        std::fs::create_dir(name).unwrap();
    }

    let base_url = format!("http://ccs.cdn.c.shop.nintendowifi.net/ccs/download/{}", title_id);

    match download_file(&format!("{}/{}", base_url, "tmd"), &format!("{}/title.tmd", name), progress_bar, label, cancel_button) {
        Ok(_) => println!("TMD Download ok"),
        Err(e) =>  {
            println!("TMD Download error");
            return Err(e);
        },
    }

    let tmd = std::fs::File::open(format!("{}/title.tmd", name)).unwrap();
    let mut reader = std::io::BufReader::new(tmd);
    let mut u16_buf = vec![0u8; 2];
    reader.seek(std::io::SeekFrom::Start(478)).unwrap();
    reader.read_exact(&mut u16_buf).unwrap();

    let mut content_count: u16 = 0;
    for byte in &u16_buf {
        content_count = (content_count << 8) | (*byte as u16);
    }

    reader.seek(std::io::SeekFrom::Start(476)).unwrap();
    reader.read_exact(&mut u16_buf).unwrap();

    let mut title_version: u16 = 0;
    for byte in &u16_buf {
        title_version = (title_version << 8) | (*byte as u16);
    }

    create_ticket(title_id, &generate_key(title_id), title_version, &format!("{}/title.tik", name));

    let mut u32_buf = vec![0u8; 4];

    for content in 0..content_count {
        if !get_queue_cancelled() {
            let offset = 2820 + (48 * content);
            reader.seek(std::io::SeekFrom::Start(offset.into())).unwrap();
            reader.read_exact(&mut u32_buf).unwrap();
            let mut id: u32 = 0;
            for byte in &u32_buf {
                id = (id << 8) | (*byte as u32);
            }
            download_file(&format!("{}/{:08x}", base_url, id), &format!("{}/{:08x}.app", name, id), progress_bar, label, cancel_button).unwrap();
            let mut has_hash_buffer = vec![0u8; 1];
            reader.seek(std::io::SeekFrom::Start((offset + 7).into())).unwrap();
            reader.read_exact(&mut has_hash_buffer).unwrap();
            let has_hash = (has_hash_buffer[0] & 0x2) == 2;
            if has_hash {
                download_file(&format!("{}/{:08x}.h3", base_url, id), &format!("{}/{:08x}.h3", name, id), progress_bar, label, cancel_button).unwrap();
            }
        }
    }

    return Ok(());
}