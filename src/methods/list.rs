// ****************************************************************************
// List bookmarks under a certain directory
// ****************************************************************************

use super::common;
use std::fs;
use std::path::PathBuf;

pub fn run(args: Vec<String>) {
  let bookmarks = if args.len() > 2 {
    common::get_toml_bookmark_files(Some(args[2].clone()))
  } else {
    common::get_toml_bookmark_files(None)
  };
  if bookmarks.is_empty() {
    eprintln!("No bookmarks found.");
    return;
  }
  let mut divisor = " :: ".to_string();

  // Parse args to find --divisor value
  let mut args_iter = args.iter();
  while let Some(arg) = args_iter.next() {
    if arg == "--divisor" {
      if let Some(value) = args_iter.next() {
        divisor = value.to_string();
      }
    } else if arg.starts_with("--divisor=") {
      if let Some(value) = arg.split_once('=').map(|x| x.1) {
        divisor = value.to_string();
      }
    }
  }
  let formatted = bookmarks.iter().map(|path| {
    let mut current_path = common::get_bookmark_store_dir_path();
    current_path.push(PathBuf::from(path));
    let full_path = format!("{}.toml", &current_path.display());
    let url =
      extract_url_from_toml(&full_path).unwrap_or_else(|_| "N/A".to_string());
    format!("{}{}{}", path, divisor, url)
  });
  for line in formatted {
    println!("{}", line);
  }
}

fn extract_url_from_toml(
  path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
  let content = fs::read_to_string(path)?;
  let toml: toml::Value = toml::from_str(&content)?;
  toml
    .get("url")
    .and_then(|v| v.as_str())
    .map(String::from)
    .ok_or_else(|| "Missing or invalid `url`".into())
}
