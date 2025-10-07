mod methods;
#[cfg(test)]
mod tests;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: tempesta <command> [options]");
    std::process::exit(1);
  }
  let command = &args[1];
  match command.as_str() {
    "add" | "a" => methods::add::run(args),
    "completion" => methods::completion::run(args),
    "config" | "c" => methods::config::run(),
    "edit" | "e" => methods::edit::run(args),
    "init" | "i" => methods::init::run(),
    "list" | "l" => methods::list::run(args),
    "move" | "m" => methods::r#move::run(args),
    "open" | "o" => methods::open::run(args),
    "remove" | "r" => methods::remove::run(args),
    "update" | "u" => methods::update::run(args),
    "--version" | "-v" => print_version(),
    _ => {
      eprintln!("Unknown command: {}", command);
      eprintln!(
        "Available commands: [a]dd, [e]dit, [l]ist, [m]ove, [o]pen, [r]emove, [u]pdate"
      );
      std::process::exit(1);
    }
  }
  std::process::exit(0);
}

// ****************************************************************************
// Print the version of the package
// ****************************************************************************
fn print_version() {
  println!("Tempesta version: {}", env!("CARGO_PKG_VERSION"));
}
