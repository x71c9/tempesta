// ****************************************************************************
// Remove a bookmark
// ****************************************************************************

use super::common::{self, PanicOnError};
use std::fs;
use std::io::{self, Write};

pub fn run(args: Vec<String>) {
  if args.len() < 3 {
    eprintln!("Usage: tempesta remove <path>");
    std::process::exit(1);
  }
  let relative_path = &args[2];
  let toml_file_path = common::get_bookmark_file_path(relative_path);
  if toml_file_path.exists() {
    fs::remove_file(&toml_file_path).panic_on_error("Failed to remove file");
    println!("Bookmark removed successfully as {}", &relative_path);
    let mut parent_dir = toml_file_path.parent();
    while let Some(dir) = parent_dir {
      if fs::remove_dir(dir).is_ok() {
        parent_dir = dir.parent();
      } else {
        break;
      }
    }
    let comment = format!("Remove bookmark {}", &relative_path);
    common::git_commit(&comment);
    return;
  }
  let mut given_path = common::get_bookmark_store_dir_path();
  given_path.push(relative_path);
  if !given_path.is_dir() {
    eprintln!("Bookmark not found: {}", toml_file_path.display());
    return;
  }
  let prompt_message = format!(
    "Bookmark not found as a file, but '{}' is a directory. \
    Do you want to delete it and all its bookmarks? [Y/n] ",
    relative_path
  );
  print!("{}", prompt_message);
  let mut input = String::new();
  io::stdout()
    .flush()
    .panic_on_error("Cannot flush prompt delete dir");
  io::stdin()
    .read_line(&mut input)
    .panic_on_error("Cannot read input delete dir");
  let input = input.trim().to_lowercase();
  if input.is_empty() || input == "y" || input == "yes" {
    fs::remove_dir_all(&given_path)
      .panic_on_error("Failed to remove directory");
    let comment =
      format!("Removed directory {} and all bookmarks", relative_path);
    common::git_commit(&comment);
    println!("Directory and all bookmarks removed: {}", relative_path);
    return;
  }
  println!("Operation canceled.");
}
