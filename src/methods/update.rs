// ****************************************************************************
// Update a bookmark
// ****************************************************************************

use super::common;

pub fn run(args: Vec<String>) {
  if args.len() < 4 {
    eprintln!("Usage: tempesta update <path> <url> [tags...]");
    std::process::exit(1);
  }
  let relative_path = &args[2];
  common::validate_path(relative_path);
  let toml_file_path = common::get_bookmark_file_path(relative_path);
  if !toml_file_path.exists() {
    eprintln!("Path {:?} do not exists", &toml_file_path.to_str());
    std::process::exit(1);
  }
  let url = &args[3];
  common::validate_url(url);
  let tags = &args[4..].to_vec();
  common::store_bookmark(&toml_file_path, url, tags);
  let comment = format!("Update bookmark {}", &relative_path);
  common::git_commit(&comment);

  println!("Bookmark updated successfully as {}", &relative_path);
}

