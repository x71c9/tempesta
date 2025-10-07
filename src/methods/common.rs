use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::config;

#[derive(Serialize, Deserialize)]
pub struct Bookmark {
  pub url: String,
  pub tags: Vec<String>,
}

pub trait PanicOnError<T> {
  fn panic_on_error(self, msg: &str) -> T;
}

// Implement for Result<T, E>
impl<T, E: std::fmt::Display> PanicOnError<T> for Result<T, E> {
  fn panic_on_error(self, msg: &str) -> T {
    self.unwrap_or_else(|e| panic!("{}: {}", msg, e))
  }
}

// Implement for Option<T>
impl<T> PanicOnError<T> for Option<T> {
  fn panic_on_error(self, msg: &str) -> T {
    self.unwrap_or_else(|| panic!("{}", msg))
  }
}

pub fn get_toml_bookmark_files(sub_path: Option<String>) -> Vec<String> {
  let root_dir = get_bookmark_store_dir_path();
  let search_dir = match &sub_path {
    Some(sub) => {
      let mut d = root_dir.clone();
      d.push(sub);
      d
    }
    None => root_dir.clone(),
  };
  let mut bookmarks = Vec::new();
  fn visit_dir(dir: &PathBuf, root_dir: &PathBuf, bookmarks: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
          visit_dir(&path, root_dir, bookmarks); // recurse
        } else if path.is_file()
          && path.extension().is_some_and(|ext| ext == "toml")
        {
          if let Ok(relative_path) = path.strip_prefix(root_dir) {
            if let Some(relative_str) = relative_path.to_str() {
              let without_extension = relative_str.trim_end_matches(".toml");
              bookmarks.push(without_extension.to_string());
            }
          }
        }
      }
    }
  }
  visit_dir(&search_dir, &root_dir, &mut bookmarks);
  if bookmarks.is_empty() {
    eprintln!("No .toml files found in {:?}", search_dir);
  }
  bookmarks
}

pub fn get_bookmark_store_dir_path() -> PathBuf {
  let config = config::load_config();
  let expanded_dir = expand_tilde(&config.dir);
  fs::create_dir_all(&expanded_dir)
    .panic_on_error("Failed to create bookmark store");
  expanded_dir
}

pub fn expand_tilde(path: &str) -> PathBuf {
  if path == "~" {
    return dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
  } else if path.starts_with("~/") {
    if let Some(home) = dirs::home_dir() {
      return home.join(path.trim_start_matches("~/"));
    }
  }
  PathBuf::from(path)
}

pub fn validate_path(relative_path: &str) {
  let re =
    Regex::new(r"^[a-zåäöA-ZÅÄÖ0-9_/.-]+$").panic_on_error("Invalid path");
  if !re.is_match(relative_path) {
    panic!("Invalid path. Please avoid spaces and special characters.");
  }
}

pub fn validate_url(url: &str) {
  let re = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$")
    .panic_on_error("Invalid url format");
  if !re.is_match(url) {
    panic!(
      "Invalid URL. Please use a proper format (e.g., https://example.com)."
    );
  }
}

pub fn get_bookmark_file_path(relative_path: &String) -> PathBuf {
  let mut bookmark_store_dir_path = get_bookmark_store_dir_path();
  let relative_path_buf = PathBuf::from(relative_path);
  let file_name = relative_path_buf
    .file_name()
    .panic_on_error("Invalid path provided")
    .to_string_lossy()
    .to_string()
    + ".toml";
  let parent_path = relative_path_buf
    .parent()
    .map(|p| p.to_path_buf())
    .unwrap_or_else(|| PathBuf::from("."));
  bookmark_store_dir_path.push(parent_path);
  fs::create_dir_all(&bookmark_store_dir_path)
    .panic_on_error("Failed to create directory");
  bookmark_store_dir_path.push(file_name);
  bookmark_store_dir_path
}

pub fn store_bookmark(toml_file_path: &PathBuf, url: &str, tags: &[String]) {
  let bookmark = Bookmark {
    url: url.to_owned(),
    tags: tags.to_owned(),
  };
  let toml_content =
    toml::to_string(&bookmark).panic_on_error("Failed to serialize bookmark");
  fs::write(toml_file_path, toml_content)
    .panic_on_error("Failed to write bookmark file");
  println!("Bookmark file stored at {}", toml_file_path.display())
}

fn push_to_origin() {
  let config = config::load_config();
  if config.remote.is_none() {
    return;
  }
  println!("Pushing changes to remote origin...");
  git_command(&["push", "-u", "--all"], "Cannot push to origin");
}

pub fn git_commit(comment: &str) {
  git_command(&["add", "-A"], "Failed to add file to git stage");
  git_command(&["commit", "-m", comment], "Failed to commit to git");
  push_to_origin();
}

pub fn git_command(args: &[&str], error_message: &str) {
  let config = config::load_config();
  if !config.git {
    return;
  }
  let bookmark_store_dir_path = get_bookmark_store_dir_path();
  run_command("git", args, &bookmark_store_dir_path, error_message);
}

pub fn run_command(
  cmd: &str,
  args: &[&str],
  dir: &std::path::Path,
  error_message: &str,
) {
  Command::new(cmd)
    .args(args)
    .current_dir(dir)
    .output()
    .panic_on_error(error_message);
}
