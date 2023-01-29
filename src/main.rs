use std::fs;
use std::path::{Path, PathBuf};

mod args;
mod http;

const MASTER_URL: &str = "https://updater.xlabs.dev";
const IW4X_RAWFILES_API_URL: &str =
    "https://api.github.com/repos/XLabsProject/iw4x-rawfiles/releases/latest";
const IW4X_RAWFILES_UPDATE_URL: &str =
    "https://github.com/XLabsProject/iw4x-rawfiles/releases/latest/download/release.zip";
const IW4X_RAWFILES_VERSION_FILE: &str = ".version.json";
const UPDATER_CONFIG_FILE: &str = "xlabs-updater.json";

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

fn load_config(path: &PathBuf) -> serde_json::Value {
    if path.exists() {
        let config: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        return config;
    }
    serde_json::Value::Null
}

fn save_config(path: &PathBuf, config: &serde_json::Value) {
    if !path.exists() {
        fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
    }
}

fn version_to_int(version: &str) -> u32 {
    version.replace(['v', '.', '\"'], "").parse().unwrap()
}

fn parse_json_get_value(json: &str, key: &str) -> String {
    let json: serde_json::Value = serde_json::from_str(json).unwrap();
    json[key].to_string()
}

fn update(path: PathBuf, include_launcher: bool) {
    println!("Getting X Labs file list");

    let cdn_info: Vec<CdnFile> = serde_json::from_str(&http::get_body_string(
        format!("{}/{}", MASTER_URL, "files.json").as_str(),
    ))
    .unwrap();

    for file in cdn_info {
        if !include_launcher && (file.name.starts_with("launcher") || file.name.starts_with("cef"))
        {
            println!("Skipping {}", file.name);
            continue;
        }

        let file_path = path.join(&file.name);
        if file_path.exists() {
            let file_sha1 = file_get_sha1(&file_path);
            if file_sha1.to_uppercase() == file.hash {
                println!("Checked {}", file.name);
                continue;
            }
        }
        println!("Downloading {}", file.name);
        download(file, &file_path)
    }
}

fn update_iw4x_rawfiles(iw4x_path: PathBuf) {
    let iw4x_rawfiles_version_local = &parse_json_get_value(
        &fs::read_to_string(iw4x_path.join(IW4X_RAWFILES_VERSION_FILE))
            .unwrap_or_else(|_| "{\"rawfile_version\":\"v0.0.0\"}".to_string()),
        "rawfile_version",
    );

    let iw4x_rawfiles_version_remote =
        &parse_json_get_value(&http::get_body_string(IW4X_RAWFILES_API_URL), "tag_name");

    if version_to_int(iw4x_rawfiles_version_local) >= version_to_int(iw4x_rawfiles_version_remote) {
        println!("iw4x rawfiles are up to date");
        return;
    }

    println!("Downloading iw4x rawfiles");
    let update_url = IW4X_RAWFILES_UPDATE_URL;
    let temp_file = std::env::temp_dir().join("release.zip");
    http::download_file(update_url, &temp_file);

    println!("Unpacking iw4x rawfiles to {}", iw4x_path.display());
    let mut archive = zip::ZipArchive::new(fs::File::open(&temp_file).unwrap()).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = iw4x_path.join(file.name());

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!("Unpacking {}", file.name());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    fs::remove_file(temp_file).unwrap();

    fs::write(
        iw4x_path.join(IW4X_RAWFILES_VERSION_FILE),
        format!("{{\"rawfile_version\":{}}}", iw4x_rawfiles_version_remote),
    )
    .expect("Error writing iw4x rawfiles version file");
}

fn main() {
    let mut args = args::get();
    let config_path = Path::new(&args.directory).join(UPDATER_CONFIG_FILE);
    let mut config = load_config(&config_path);

    update(PathBuf::from(&args.directory), args.launcher);

    if args.iw4x_path.is_empty() {
        if config["iw4x_path"].is_string() {
            let iw4x_path = config["iw4x_path"].as_str().unwrap();
            args.iw4x_path = iw4x_path.to_string();
            println!("Using iw4x path from config: {}", iw4x_path);
        } else {
            println!("To update iw4x please specify the game path using --iw4x-path at least once");
            return;
        }
    } else {
        config["iw4x_path"] = serde_json::Value::String(args.iw4x_path.to_string());
        save_config(&config_path, &config);
    }
    update_iw4x_rawfiles(PathBuf::from(args.iw4x_path));
}
