// ****************************************************************************
// Get a bookmark and print it
// ****************************************************************************

use super::common;

pub fn run(args: Vec<String>) {
  let relative_path = if args.len() < 3 {
    // No path provided, try to invoke finder
    eprintln!("Usage: tempesta get <path>");
    std::process::exit(1);
  } else {
    args[2].clone()
  };
  common::validate_path(&relative_path);
  let url = common::get_url(&relative_path);
  common::validate_url(&url);
  println!("{}", url);
}
