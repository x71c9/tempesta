use super::common::{self, PanicOnError};
use std::io::{self, Write};

// ****************************************************************************
// Add a bookmark
// ****************************************************************************
pub fn run(args: Vec<String>) {
  if args.len() < 4 {
    eprintln!("Usage: tempesta add <path> <url> [tags...]");
    std::process::exit(1);
  }
  let relative_path = &args[2];
  common::validate_path(relative_path);
  let toml_file_path = common::get_bookmark_file_path(relative_path);
  if toml_file_path.exists() {
    print!(
      "Bookmark already exists at {}. Overwrite? (y/N): ",
      toml_file_path.display()
    );
    io::stdout()
      .flush()
      .panic_on_error("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .panic_on_error("Failed to read input");

    let input = input.trim().to_lowercase();
    if input.is_empty() || ["n", "no"].contains(&input.as_str()) {
      println!("Operation cancelled.");
      return;
    }
    if !["y", "yes"].contains(&input.as_str()) {
      println!("Invalid input. Operation cancelled.");
      return;
    }
    println!("Overwriting file...");
  }
  let url = &args[3];
  common::validate_url(url);
  let tags = &args[4..].to_vec();
  common::store_bookmark(&toml_file_path, url, tags);
  let comment = format!("Add bookmark {}", &relative_path);
  common::git_commit(&comment);
  println!("Bookmark added successfully as {}", &relative_path);
}
