// ****************************************************************************
// Edit a bookmark
// ****************************************************************************

use super::common::{self, PanicOnError};
use std::env;
use std::fs;
use std::process::Command;

pub fn run(args: Vec<String>) {
  if args.len() < 3 {
    eprintln!("Usage: tempesta edit <path>");
    std::process::exit(1);
  }
  let relative_path = &args[2];
  common::validate_path(relative_path);
  let toml_file_path = common::get_bookmark_file_path(relative_path);
  if !toml_file_path.exists() {
    eprintln!("Bookmark file does not exist: {}", toml_file_path.display());
    std::process::exit(1);
  }
  // Get preferred editor from $EDITOR, or default to nano
  let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
  // Store last modified timestamp before editing
  let metadata_before = fs::metadata(&toml_file_path)
    .and_then(|m| m.modified())
    .ok();
  // Open the file in the preferred editor (blocking)
  let status = Command::new(&editor)
    .arg(&toml_file_path)
    .status()
    .panic_on_error("Failed to open editor");
  if !status.success() {
    eprintln!("Failed to edit bookmark file.");
    return;
  }
  // Check if the file was modified
  let metadata_after = fs::metadata(&toml_file_path)
    .and_then(|m| m.modified())
    .ok();
  if metadata_before != metadata_after {
    let comment = format!("Edit bookmark {}", &toml_file_path.display());
    common::git_commit(&comment);
    println!("Bookmark edited successfully as {}", &relative_path);
  } else {
    println!("No changes made.");
  }
}

