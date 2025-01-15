use std::env;
use std::fs::{create_dir_all, write};
use std::path::Path;

fn main() {
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let download_dir = Path::new(&home_dir).join("Downloads");
    let output_dir = download_dir.join("output");

    create_dir_all(&output_dir).expect("Failed to create output directory");

    for i in 0..5 {
        let response =
            reqwest::blocking::get("https://cataas.com/cat").expect("Failed to fetch cat");

        let mime_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("image/jpeg");

        let file_type = mime_type
            .split('/')
            .next_back()
            .unwrap_or("jpeg")
            .to_string();

        let cat_bytes = response.bytes().expect("Failed to read bytes");

        let file_path = output_dir.join(format!("cat-{}.{}", i, file_type));

        println!("Saving cat to {:?}...", file_path);
        write(&file_path, cat_bytes).expect("Failed to write file");
    }
}
