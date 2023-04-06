use std::io::Read;
use std::io::Seek;
use std::io::Write;

use futures_util::StreamExt;

fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

async fn download_file(client: &reqwest::Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    
    // Indicatif setup
    let pb = indicatif::ProgressBar::new(total_size);
    pb.set_style(indicatif::ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(&format!("Downloading {}", url));

    // download chunks
    let mut file = std::fs::File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(());
}

pub async fn download_title(title_id: &str, name: &str) -> std::io::Result<()> {
    if !path_exists(name) {
        std::fs::create_dir(name).unwrap();
    }

    let base_url = format!("http://ccs.cdn.c.shop.nintendowifi.net/ccs/download/{}", title_id);

    let client = reqwest::Client::new();
    download_file(&client, &format!("{}/{}", base_url, "tmd"), &format!("{}/title.tmd", name)).await.unwrap();

    let tmd = std::fs::File::open(format!("{}/title.tmd", name))?;
    let mut reader = std::io::BufReader::new(tmd);
    let mut u16_buf = vec![0u8; 2];
    reader.seek(std::io::SeekFrom::Start(478))?;
    reader.read_exact(&mut u16_buf)?;

    let mut content_count: u16 = 0;
    for byte in &u16_buf {
        content_count = (content_count << 8) | (*byte as u16);
    }

    let mut u32_buf = vec![0u8; 4];

    for content in 0..content_count {
        let offset = 2820 + (48 * content);
        reader.seek(std::io::SeekFrom::Start(offset.into()))?;
        reader.read_exact(&mut u32_buf)?;
        let mut id: u32 = 0;
        for byte in &u32_buf {
            id = (id << 8) | (*byte as u32);
        }
        download_file(&client, &format!("{}/{:08x}", base_url, id), &format!("{}/{:08x}.app", name, id)).await.unwrap();
        let mut has_hash_buffer = vec![0u8; 1];
        reader.seek(std::io::SeekFrom::Start((offset + 7).into()))?;
        reader.read_exact(&mut has_hash_buffer)?;
        let has_hash = (has_hash_buffer[0] & 0x2) == 2;
        if has_hash {
            download_file(&client, &format!("{}/{:08x}.h3", base_url, id), &format!("{}/{:08x}.h3", name, id)).await.unwrap();
        }
    }

    return Ok(());
}

#[tokio::main]
async fn main() {
    let _ = download_title("00050000101c9500", "BOTW EUR").await;
}