// ****************************************************************************
// Move a bookmark
// ****************************************************************************

use std::fs;
use std::io::{self, Write};
use std::path::{Path};
use super::common::{self, PanicOnError};

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
    eprintln!("Path {:?} do not exists", &toml_from_file_path.to_str());
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
    &relative_path_from, &relative_path_to
  );
  common::git_commit(&comment);

  println!(
    "Bookmark moved successfully from {} to {}",
    &relative_path_from, &relative_path_to
  );
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

