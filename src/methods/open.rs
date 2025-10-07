// ****************************************************************************
// Open a bookmark in the browser
// ****************************************************************************

use super::common::{self, PanicOnError};

pub fn run(args: Vec<String>) {
  let relative_path = if args.len() < 3 {
    // No path provided, try to invoke finder
    eprintln!("Usage: tempesta open <path>");
    std::process::exit(1);
  } else {
    args[2].clone()
  };
  common::validate_path(&relative_path);
  let url = common::get_url(&relative_path);
  common::validate_url(&url);
  webbrowser::open(&url).panic_on_error("Failed to open browser");
}
