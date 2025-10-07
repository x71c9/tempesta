// ****************************************************************************
// Initialize the package. The command that must be run the first time.
// It sets the configuration:
// - store path
// - git
// - git remote url
// ****************************************************************************

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use super::common::{self, PanicOnError};
use super::config::{self, Config};

pub fn run() {
  let storage_path = prompt_valid_bookmark_store_path();
  print!("Do you want to use Git for tracking bookmarks? (Y/n): ");
  io::stdout()
    .flush()
    .panic_on_error("Failed to flush stdout");
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .panic_on_error("Failed to read input");
  let use_git = !matches!(input.trim().to_lowercase().as_str(), "n" | "no");
  let config = Config {
    git: use_git,
    remote: None,
    dir: storage_path,
  };
  config::save_config(&config);
  if use_git {
    handle_git(&config);
  }
  let config_file_path = config::get_config_file_path();
  println!(
    "Tempesta initialized successfully: {}",
    config_file_path.display()
  );
}

fn prompt_valid_bookmark_store_path() -> String {
  loop {
    let mut storage_path = String::new();
    print!("Where do you want to store the bookmarks? [~/.bookmark-store]: ");
    io::stdout().flush().unwrap();
    io::stdin()
      .read_line(&mut storage_path)
      .expect("Failed to read input");
    let storage_path = storage_path.trim();

    // Default path if the user input is empty
    let storage_path = if storage_path.is_empty() {
      let home_dir = dirs::home_dir().expect("Could not find home directory");
      let mut default_dir = home_dir;
      default_dir.push(".bookmark-store");
      default_dir.to_string_lossy().into_owned()
    } else {
      let expanded = common::expand_tilde(storage_path);
      let path = Path::new(&expanded);

      // Check if the path is valid (absolute or has a parent directory)
      if !(path.is_absolute() || path.parent().is_some()) {
        println!("Invalid path format. Please enter a valid absolute or relative path.");
        continue;
      }

      // Check if the path has write permission
      if !check_write_permission(path) {
        println!(
          "No write permission for the specified path: {}",
          expanded.to_string_lossy()
        );
        continue;
      }

      expanded.to_string_lossy().into_owned()
    };

    return storage_path;
  }
}

fn handle_git(previous_config: &Config) {
  let git_remote = prompt_remote_url();
  let bookmark_store_dir_path = common::get_bookmark_store_dir_path();

  common::run_command(
    "git",
    &["init"],
    &bookmark_store_dir_path,
    "Failed to initialize Git repository",
  );
  println!(
    "Git repository initialized at {}",
    bookmark_store_dir_path.display()
  );

  if let Some(remote) = &git_remote {
    let branch_name = prompt_branch_name();
    common::run_command(
      "git",
      &["remote", "add", "origin", remote],
      &bookmark_store_dir_path,
      "Failed to add remote repository",
    );
    println!("Git remote repository set to {}", remote);
    common::run_command(
      "git",
      &["pull", "origin", &branch_name],
      &bookmark_store_dir_path,
      "Failed to pull from origin",
    );
  }

  let config = Config {
    git: true,
    remote: git_remote,
    dir: previous_config.dir.clone(),
  };
  config::save_config(&config);
}

fn prompt_remote_url() -> Option<String> {
  print!("Enter the remote repository URI (leave empty for no remote): ");
  io::stdout()
    .flush()
    .panic_on_error("Failed to flush stdout");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .panic_on_error("Failed to read input");
  let trimmed = input.trim();

  if trimmed.is_empty() {
    None
  } else {
    Some(trimmed.to_string())
  }
}

fn prompt_branch_name() -> String {
  print!("Enter the branch name to pull from [master]: ");
  io::stdout()
    .flush()
    .panic_on_error("Failed to flush stdout");
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .panic_on_error("Failed to read input");
  let trimmed = input.trim();
  if trimmed.is_empty() {
    "master".to_string() // Default to "master" if no input is given
  } else {
    trimmed.to_string()
  }
}

fn check_write_permission(path: &Path) -> bool {
  // Get the parent directory or use the path itself if it's already a directory
  let parent = if path.is_dir() {
    path
  } else {
    path.parent().unwrap_or(path)
  };

  let test_file = parent.join(".permission_check");
  match File::create(&test_file) {
    Ok(_) => {
      let _ = fs::remove_file(&test_file);
      true
    }
    Err(_) => false,
  }
}
