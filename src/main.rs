mod methods;

#[cfg(test)]
mod tests;

use methods::common::CONFIG_FILE_PATH;
use std::env;
use std::path::PathBuf;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut config_path: Option<PathBuf> = None;
  let mut processed_args: Vec<String> = Vec::new();

  // Manually parse for --config or -c flag
  let mut i = 0;
  while i < args.len() {
    if args[i] == "--config" || args[i] == "-c" {
      if i + 1 < args.len() {
        config_path = Some(PathBuf::from(&args[i + 1]));
        i += 2; // Consume flag and value
        continue;
      } else {
        eprintln!("Error: --config requires a path argument.");
        std::process::exit(1);
      }
    }
    processed_args.push(args[i].clone());
    i += 1;
  }

  // If --config was not provided, check TEMPESTA_CONFIG environment variable
  if config_path.is_none() {
    if let Ok(env_config) = env::var("TEMPESTA_CONFIG") {
      config_path = Some(PathBuf::from(env_config));
    }
  }

  // Initialize the global CONFIG_FILE_PATH
  if let Some(path) = config_path {
    CONFIG_FILE_PATH
      .set(path)
      .expect("Failed to set config file path");
  }

  // Now use processed_args for command dispatch
  if processed_args.len() < 2 {
    eprintln!("Usage: tempesta <command> [options]");
    std::process::exit(1);
  }
  let command = &processed_args[1];
  match command.as_str() {
    "add" | "a" => methods::add::run(processed_args),
    "completion" => methods::completion::run(processed_args),
    "config" | "c" => methods::config::run(),
    "edit" | "e" => methods::edit::run(processed_args),
    "get" | "g" => methods::get::run(processed_args),
    "init" | "i" => methods::init::run(),
    "list" | "l" | "ls" => methods::list::run(processed_args),
    "move" | "m" | "mv" => methods::r#move::run(processed_args),
    "open" | "o" => methods::open::run(processed_args),
    "remove" | "r" | "rm" => methods::remove::run(processed_args),
    "update" | "u" => methods::update::run(processed_args),
    "--help" | "-h" => print_help(),
    "--version" | "-v" => print_version(),
    _ => {
      eprintln!("Unknown command: {}", command);
      eprintln!(
                "Available commands: [a]dd, [e]dit, [i]nit, [l]ist, [m]ove, [o]pen, [r]emove, [u]pdate"
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

// ****************************************************************************
// Print help
// ****************************************************************************
fn print_help() {
  println!("Tempesta - The lightest and fastest CLI for managing bookmarks");
  println!();
  println!("USAGE:");
  println!("    tempesta <COMMAND> [OPTIONS]");
  println!();
  println!("COMMANDS:");
  println!("    add, a          Add a new bookmark");
  println!("    completion      Generate shell completion");
  println!("    config, c       Show configuration");
  println!("    edit, e         Edit a bookmark");
  println!("    get, g          Get bookmark URL");
  println!("    init, i         Initialize configuration");
  println!("    list, l, ls     List all bookmarks");
  println!("    move, m, mv     Move/rename a bookmark");
  println!("    open, o         Open a bookmark in browser");
  println!("    remove, r, rm   Remove a bookmark");
  println!("    update, u       Update a bookmark");
  println!();
  println!("OPTIONS:");
  println!("    --config, -c <PATH>    Use custom config file path");
  println!("    --help, -h             Show this help message");
  println!("    --version, -v          Show version information");
}
