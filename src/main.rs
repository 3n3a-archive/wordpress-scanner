#[macro_use]
extern crate rocket;

use pandoc;
use rocket::fs::{FileServer, NamedFile, Options};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use std::env;
use once_cell::sync::Lazy;

const ASSETS_PATH: Lazy<String> = Lazy::new(||env::var("ASSETS_PATH").unwrap_or(String::from("files")) );
const PANDOC_IN_PATH: Lazy<String> = Lazy::new(||env::var("PANDOC_IN_PATH").unwrap_or(String::from("docs")) );
const PANDOC_OUT_PATH: Lazy<String> = Lazy::new(||env::var("PANDOC_OUT_PATH").unwrap_or(String::from("docs/generated")) );

#[launch]
fn rocket() -> _ {
    let file_server_options = Options::None | Options::Index | Options::NormalizeDirs;
    rocket::build()
        .mount("/assets", FileServer::new((*ASSETS_PATH).clone(), file_server_options))
        .mount("/", routes![files])
}

#[get("/<file..>", rank = 40)]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let file_stem: &str;
    let file_extension: &str;

    // if path is Index try to display README
    let path_is_index = (file == PathBuf::from(""))
        || (file == PathBuf::from("/"))
        || file.ends_with("/index.html")
        || file.ends_with("/index");

    debug!("Is index route {}", path_is_index);
    if !file.is_file() && path_is_index {
        file_stem = "README";
        file_extension = "md";
    } else {
        file_stem = file.file_stem().unwrap().to_str().unwrap();
        file_extension = file
            .extension()
            .unwrap_or(Path::new("").as_os_str())
            .to_str()
            .unwrap_or("");
    }

    let file_output_path = match file_extension {
        "html" | "md" => convert_md_to_html(file_stem),
        _ => PathBuf::new(),
    };

    NamedFile::open(file_output_path.as_path()).await.ok()
}

fn convert_md_to_html(file_stem: &str) -> PathBuf {
    let file_input_path: PathBuf =
        PathBuf::from_iter([(*PANDOC_IN_PATH).clone(), format!("{}.md", file_stem)].iter());
    let file_output_path: PathBuf =
        PathBuf::from_iter([(*PANDOC_OUT_PATH).clone(), format!("{}.html", file_stem)].iter());

    if !file_input_path.is_file() {
        return PathBuf::new();
    }

    let file_input_modified = file_input_path.metadata().unwrap().modified().unwrap();
    let file_output_modified = if file_output_path.is_file() {
        file_output_path.metadata().unwrap().modified().unwrap()
    } else {
        UNIX_EPOCH
    };

    let output_nonexistent = !file_output_path.is_file();
    let output_outdated =
        file_output_path.is_file() && (file_input_modified >= file_output_modified);

    if output_nonexistent || output_outdated {
        debug!(
            "Pandoc for Markdown at {}",
            file_output_path.to_str().unwrap()
        );
        debug!(
            "Reason for Conversion {}",
            if output_nonexistent {
                "no file in output"
            } else {
                "old version in output"
            }
        );

        let mut pandoc = pandoc::new();
        pandoc.add_input(file_input_path.to_str().unwrap());
        pandoc.set_output(pandoc::OutputKind::File(file_output_path.clone()));
        pandoc.add_option(pandoc::PandocOption::Standalone);
        // pandoc.set_show_cmdline(true);
        pandoc.execute().unwrap();
    }

    file_output_path
}
