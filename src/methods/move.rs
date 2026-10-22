// ****************************************************************************
// Move a bookmark
// ****************************************************************************

use super::common::{self, PanicOnError};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(args: Vec<String>) {
  if args.len() < 4 {
    eprintln!("Usage: tempesta move <path-from> <path-to>");
    std::process::exit(1);
  }

  let relative_path_from = &args[2];
  common::validate_path(relative_path_from);
  let relative_path_to = &args[3];
  common::validate_path(relative_path_to);

  let toml_from_file_path = common::get_bookmark_file_path(relative_path_from);
  if !toml_from_file_path.exists() {
    let mut dir_from_path = common::get_bookmark_store_dir_path();
    dir_from_path.push(relative_path_from);
    if dir_from_path.is_dir() {
      move_directory(&dir_from_path, relative_path_from, relative_path_to);
      return;
    }
    eprintln!("Path {:?} do not exists", toml_from_file_path.to_str());
    std::process::exit(1);
  }

  let toml_to_file_path = if relative_path_to.ends_with('/') {
    let directory_path = Path::new(relative_path_to);
    let file_name = toml_from_file_path.file_stem().unwrap();
    let full_path = &directory_path.join(file_name).display().to_string();
    common::get_bookmark_file_path(full_path)
  } else {
    common::get_bookmark_file_path(relative_path_to)
  };

  if toml_to_file_path.exists() && !prompt_for_overwrite(&toml_to_file_path) {
    println!("Move operation aborted.");
    std::process::exit(0);
  }

  if let Some(parent) = toml_to_file_path.parent() {
    if !parent.exists() {
      fs::create_dir_all(parent)
        .panic_on_error("Failed to create destination directory")
    }
  }

  fs::rename(&toml_from_file_path, &toml_to_file_path)
    .panic_on_error("Failed to move bookmark file");

  // After successful move, cleanup empty parent directories
  if let Some(parent) = toml_from_file_path.parent() {
    cleanup_empty_parents(parent)
      .panic_on_error("Warning: Failed to clean up empty directories")
  }

  let comment = format!(
    "Move bookmark from {} to {}",
    relative_path_from, relative_path_to
  );
  common::git_commit(&comment);

  println!(
    "Bookmark moved successfully from {} to {}",
    relative_path_from, relative_path_to
  );
}

fn move_directory(
  dir_from_path: &Path,
  relative_path_from: &str,
  relative_path_to: &str,
) {
  let store_dir_path = common::get_bookmark_store_dir_path();
  let canonical_store = store_dir_path
    .canonicalize()
    .panic_on_error("Failed to resolve bookmark store");
  let canonical_from = dir_from_path
    .canonicalize()
    .panic_on_error("Failed to resolve source directory");
  if canonical_from == canonical_store {
    eprintln!("Cannot move the bookmark store root");
    std::process::exit(1);
  }

  // Like `mv`: a trailing slash or an existing directory means "move into"
  let mut dir_to_path = canonical_store.join(relative_path_to);
  if relative_path_to.ends_with('/') || dir_to_path.is_dir() {
    let dir_name = canonical_from.file_name().unwrap();
    dir_to_path.push(dir_name);
  }
  if dir_to_path.starts_with(&canonical_from) {
    eprintln!("Cannot move a directory into itself");
    std::process::exit(1);
  }

  if dir_to_path.exists() {
    merge_directory(&canonical_from, &dir_to_path);
    // Fails when bookmarks were kept, which leaves the directory in place
    let _ = fs::remove_dir(&canonical_from);
  } else {
    if let Some(parent) = dir_to_path.parent() {
      fs::create_dir_all(parent)
        .panic_on_error("Failed to create destination directory");
    }
    fs::rename(&canonical_from, &dir_to_path)
      .panic_on_error("Failed to move directory");
  }

  // After successful move, cleanup empty parent directories
  if let Some(parent) = canonical_from.parent() {
    cleanup_empty_parents(parent)
      .panic_on_error("Warning: Failed to clean up empty directories")
  }

  let comment = format!(
    "Move directory from {} to {}",
    relative_path_from, relative_path_to
  );
  common::git_commit(&comment);

  println!(
    "Directory moved successfully from {} to {}",
    relative_path_from, relative_path_to
  );
}

/// Moves every entry of `from` into `to`, asking before overwriting an
/// existing bookmark. Entries that are not overwritten stay in `from`.
fn merge_directory(from: &Path, to: &Path) {
  fs::create_dir_all(to)
    .panic_on_error("Failed to create destination directory");
  let entries =
    fs::read_dir(from).panic_on_error("Failed to read source directory");
  for entry in entries.filter_map(|e| e.ok()) {
    let source = entry.path();
    let destination = to.join(entry.file_name());
    if source.is_dir() {
      merge_directory(&source, &destination);
      // Fails when bookmarks were kept, which leaves the directory in place
      let _ = fs::remove_dir(&source);
    } else if !destination.exists() || prompt_for_overwrite(&destination) {
      fs::rename(&source, &destination)
        .panic_on_error("Failed to move bookmark file");
    }
  }
}

fn prompt_for_overwrite(destination: &Path) -> bool {
  print!(
    "A bookmark already exists at {}. Overwrite? [Y/n]: ",
    destination.display()
  );
  io::stdout().flush().expect("Failed to flush stdout");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read user input");

  let trimmed = input.trim().to_lowercase();

  match trimmed.as_str() {
    "" | "y" | "yes" => true, // Default to yes on Enter
    "n" | "no" => false,
    _ => {
      println!("Invalid input, assuming 'no'.");
      false
    }
  }
}

/// Recursively removes parent directories if they are empty.
/// Stops once a non-empty directory is found or the root is reached.
fn cleanup_empty_parents(starting_dir: &Path) -> std::io::Result<()> {
  let mut current = starting_dir.to_path_buf();
  loop {
    // If directory does not exist (already removed), we just break.
    if !current.exists() {
      break;
    }
    // Check if the directory is empty.
    let is_empty = fs::read_dir(&current)?.next().is_none();
    if is_empty {
      // Try to remove the directory.
      fs::remove_dir(&current)?;
      // Move to parent directory for the next check.
      if !current.pop() {
        break; // Reached the root.
      }
    } else {
      break; // Stop if we hit a non-empty directory.
    }
  }
  Ok(())
}
