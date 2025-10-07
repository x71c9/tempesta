// ****************************************************************************
// Print completion script according to the shell provided as arg or the one 
// that was set in the $SHELL environmental variable
// ****************************************************************************

use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shell {
  Bash,
  Zsh,
  Fish,
}

impl Shell {
  pub fn from_shell_str(s: &str) -> Option<Self> {
    match s {
      "bash" => Some(Shell::Bash),
      "zsh" => Some(Shell::Zsh),
      "fish" => Some(Shell::Fish),
      _ => None,
    }
  }
  pub fn completion_script(&self) -> &'static str {
    match self {
      Shell::Bash => BASH_COMPLETION,
      Shell::Zsh => ZSH_COMPLETION,
      Shell::Fish => FISH_COMPLETION,
    }
  }
}

const BASH_COMPLETION: &str =
  include_str!("../completions/tempesta-completion.bash.sh");
const ZSH_COMPLETION: &str =
  include_str!("../completions/tempesta-completion.zsh.sh");
const FISH_COMPLETION: &str =
  include_str!("../completions/tempesta-completion.fish.sh");

pub fn run(args: Vec<String>) {
  let detected_shell = detect_shell()
    .as_deref()
    .and_then(Shell::from_shell_str)
    .unwrap_or(Shell::Bash);
  let mut selected_shell = detected_shell;
  if args.len() > 2 {
    if let Some(shell) = Shell::from_shell_str(&args[2]) {
      selected_shell = shell;
    }
  }
  let script = selected_shell.completion_script();
  println!("{}", script)
}

fn detect_shell() -> Option<String> {
  env::var("SHELL").ok().and_then(|shell_path| {
    shell_path.split('/').next_back().map(|s| s.to_string())
  })
}

