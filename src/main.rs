mod keygen;
mod progress_dialog;

use progress_dialog::Progress;
use gtk::{prelude::*, ProgressBar, Label, Application};

use std::io::Read;
use std::io::Seek;
use std::io::Cursor;

use reqwest::Url;
use std::fs::File;
use std::io::Write;

use keygen::generate_key;

fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

fn char_to_int(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        'A'..='F' => c as u8 - b'A' + 10,
        'a'..='f' => c as u8 - b'a' + 10,
        _ => 0,
    }
}

fn hex_to_bytes(input: &str, output: &mut [u8]) {
    let input_len = input.len();
    let mut i = 0;
    while i < input_len {
        output[i / 2] = char_to_int(input.chars().nth(i).unwrap()) * 16
            + char_to_int(input.chars().nth(i + 1).unwrap());
        i += 2;
    }
}

fn create_ticket(title_id: &str, title_key: &str, title_version: u16, output_path: &str) {
    let mut ticket_data: [u8; 848] = [0x00, 0x01, 0x00, 0x04, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0xd1, 0x5e, 0xa5, 0xed, 0x15, 0xab, 0xe1, 0x1a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x6f, 0x6f, 0x74, 0x2d, 0x43, 0x41, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x33, 0x2d, 0x58, 0x53, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0xfe, 0xed, 0xfa, 0xce, 0x01, 0x00, 0x00, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00, 0xac, 0x00, 0x00, 0x00, 0x14, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x84, 0x00, 0x00, 0x00, 0x84, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    hex_to_bytes(title_id, &mut ticket_data[476..]);
    hex_to_bytes(title_key, &mut ticket_data[447..]);
    ticket_data[486..488].copy_from_slice(&title_version.to_be_bytes());
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "Error: The file \"{}\" couldn't be opened. {:?}\n",
                output_path,
                err
            );
            return;
        }
    };
    if let Err(err) = file.write_all(&ticket_data) {
        eprintln!("Error: Failed to write to file {}. {:?}", output_path, err);
        return;
    }
    if let Err(err) = file.sync_all() {
        eprintln!("Error: Failed to sync file {}. {:?}", output_path, err);
        return;
    }
}

fn download_file(url: &Url, path: &str, progress: &Progress) -> Result<(), reqwest::Error> {
    let mut response = reqwest::blocking::get(url.as_str())?;
    let content_length = response.content_length().unwrap_or(0);
    let mut downloaded = 0;
    let mut writer = Cursor::new(Vec::new());

    loop {
        let mut buffer = [0; 1024];
        let bytes_read = response.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        downloaded += bytes_read as u64;
        update_progress(downloaded, content_length, progress);

        writer.write_all(&buffer[..bytes_read]).unwrap();
    }

    let mut file = File::create(path).unwrap();
    file.write_all(&writer.into_inner()).unwrap();

    Ok(())
}

pub fn download_title(title_id: &str, name: &str, progress: &progress_dialog::Progress) -> std::io::Result<()> {
    if !path_exists(name) {
        std::fs::create_dir(name).unwrap();
    }

    let base_url = format!("http://ccs.cdn.c.shop.nintendowifi.net/ccs/download/{}", title_id);

    download_file( &Url::parse(&format!("{}/{}", base_url, "tmd")).unwrap(), &format!("{}/title.tmd", name), &progress).unwrap();

    let tmd = std::fs::File::open(format!("{}/title.tmd", name))?;
    let mut reader = std::io::BufReader::new(tmd);
    let mut u16_buf = vec![0u8; 2];
    reader.seek(std::io::SeekFrom::Start(478))?;
    reader.read_exact(&mut u16_buf)?;

    let mut content_count: u16 = 0;
    for byte in &u16_buf {
        content_count = (content_count << 8) | (*byte as u16);
    }

    reader.seek(std::io::SeekFrom::Start(476))?;
    reader.read_exact(&mut u16_buf)?;

    let mut title_version: u16 = 0;
    for byte in &u16_buf {
        title_version = (title_version << 8) | (*byte as u16);
    }

    create_ticket(title_id, &generate_key(title_id), title_version, &format!("{}/title.tik", name));

    let mut u32_buf = vec![0u8; 4];

    for content in 0..content_count {
        let offset = 2820 + (48 * content);
        reader.seek(std::io::SeekFrom::Start(offset.into()))?;
        reader.read_exact(&mut u32_buf)?;
        let mut id: u32 = 0;
        for byte in &u32_buf {
            id = (id << 8) | (*byte as u32);
        }
        download_file(&Url::parse(&format!("{}/{:08x}", base_url, id)).unwrap(), &format!("{}/{:08x}.app", name, id), &progress).unwrap();
        let mut has_hash_buffer = vec![0u8; 1];
        reader.seek(std::io::SeekFrom::Start((offset + 7).into()))?;
        reader.read_exact(&mut has_hash_buffer)?;
        let has_hash = (has_hash_buffer[0] & 0x2) == 2;
        if has_hash {
            download_file( &Url::parse(&format!("{}/{:08x}.h3", base_url, id)).unwrap(), &format!("{}/{:08x}.h3", name, id), &progress).unwrap();
        }
    }

    return Ok(());
}

fn update_progress(downloaded: u64, total: u64, progress: &Progress) {
    progress.progress.set_fraction((downloaded / total) as f64);
    progress.label.set_label(&format!("{:?}%", (downloaded / total) * 100));
}

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

#[tokio::main]
async fn main() {
    gtk::init().unwrap();
    let progress = Progress {
        progress: ProgressBar::new(),
        label: Label::new(None),
    };
    progress.progress.set_text(Some("Downloading..."));
    progress.label.set_text("0%");

    let window = progress_dialog::progress_dialog(&progress);

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        app.add_window(&window);
        window.show();
    });

    std::thread::spawn(move || {
        match download_title("00050000101c9500", "BOTW EUR", &progress) {
            Ok(_) => {
                println!("Download completed successfully.");
            }
            Err(e) => {
                println!("Error downloading file: {}", e);
            }
        }
    });

    app.run();
}