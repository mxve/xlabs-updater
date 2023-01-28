use std::fs;
use std::path::Path;

mod args;
mod http;

const MASTER_URL: &str = "https://updater.xlabs.dev";

#[derive(serde::Deserialize, serde::Serialize)]
struct CdnFile {
    name: String,
    size: u32,
    hash: String,
}

fn file_get_sha1(path: &Path) -> String {
    let mut sha1 = sha1_smol::Sha1::new();
    sha1.update(&fs::read(path).unwrap());
    sha1.digest().to_string()
}

fn download(file: CdnFile, file_path: &Path) {
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
        }
    }

    http::download_file(
        format!("{}/{}/{}", MASTER_URL, "data", file.name).as_str(),
        file_path,
    );
}

fn main() {
    let args = args::get();

    let cdn_info: Vec<CdnFile> = serde_json::from_str(&http::get_body_string(
        format!("{}/{}", MASTER_URL, "files.json").as_str(),
    ))
    .unwrap();

    for file in cdn_info {
        

        if !args.launcher && (file.name.starts_with("launcher") || file.name.starts_with("cef")) {
            println!("Skipping {}", file.name);
            continue;
        }

        let file_path = std::path::Path::new(&args.directory).join(&file.name);
        if file_path.exists() {
            let file_sha1 = file_get_sha1(&file_path);
            if file_sha1.to_uppercase() == file.hash {
                println!("File is up to date: {}", file.name);
                continue;
            }
        }
        println!("Downloading {}", file.name);
        download(file, &file_path)
    }
}

