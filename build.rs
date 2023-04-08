extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::io::Cursor;

use reqwest::blocking::Client;

fn fetch_titles(file_name: &str) {
    let client = Client::builder()
        .user_agent("NUSspliBuilder/2.1")
        .build()
        .unwrap();
    let response = client.get("https://napi.nbg01.v10lator.de/db").send().unwrap();
    let mut file = std::fs::File::create(file_name).unwrap();
    let mut content = Cursor::new(response.bytes().unwrap());
    std::io::copy(&mut content, &mut file).unwrap();
}

fn main() {
    fetch_titles("c-src/gtitles.c");
    println!("cargo:rerun-if-changed=c-src/gtitles.c");
    let bindings = bindgen::Builder::default()
        .header("c-src/gtitles.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("c-src/gtitles.c")
        .file("c-src/titleInfo.c")
        .flag("-Wno-trigraphs")
        .flag("-Ic-src/")
        .compile("gtitles");
    println!("cargo:rustc-link-lib=gtitles");
}