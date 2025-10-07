// ****************************************************************************
// Open a bookmark in the browser
// ****************************************************************************

use super::common::{self, PanicOnError};
use std::fs;

pub fn run(args: Vec<String>) {
  let relative_path = if args.len() < 3 {
    // No path provided, try to invoke finder
    eprintln!("Usage: tempesta open <path>");
    std::process::exit(1);
  } else {
    args[2].clone()
  };
  common::validate_path(&relative_path);
  let url = get_url(&relative_path);
  common::validate_url(&url);
  webbrowser::open(&url).panic_on_error("Failed to open browser");
}

fn get_url(relative_path: &String) -> String {
  let toml_file_path = common::get_bookmark_file_path(relative_path);
  let toml_content =
    fs::read_to_string(toml_file_path).panic_on_error("Failed to read TOML");
  let bookmark: common::Bookmark = toml::from_str(&toml_content)
    .panic_on_error("Failed to parse TOML content");
  bookmark.url
}


