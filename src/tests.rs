// use crate::add;
use assert_cmd::Command;
use std::env;
use std::path::PathBuf;

fn test_env() -> PathBuf {
  let mut tempesta_config =
    PathBuf::from(env::var("HOME").expect("HOME environment variable not set"));
  tempesta_config.push(".config/tempesta/test.toml");
  env::set_var("TEMPESTA_CONFIG", tempesta_config);
  PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
}

#[test]
fn tempesta_init() {
  let home = test_env();
  let output = "Where do you want to store the bookmarks? [~/.bookmark-store]: Do you want to use Git for tracking bookmarks? (Y/n): Tempesta initialized successfully: HOME/.config/tempesta/test.toml\n"
        .replace("HOME", home.to_str().expect("Unable to convert HOME dir to str"));
  Command::cargo_bin("tempesta")
    .unwrap()
    .arg("init")
    .write_stdin("~/.bookmark-store-test\nno\n")
    .assert()
    .success()
    .stdout(output);
  //TODO: assert file is created and looks as expected
}

#[test]
fn tempesta_add_overwrite_move_remove() {
  let home = test_env();

  // add
  let output_add = "Bookmark file stored at HOME/.bookmark-store-test/test.toml\nBookmark added successfully as test\n"
        .replace("HOME", home.to_str().expect("Unable to convert HOME dir to str"));
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["add", "test", "https://test.local", "test"])
    .assert()
    .success()
    .stdout(output_add);

  // add (again but this time overwrite)
  let output_add_overwrite = "Bookmark already exists at HOME/.bookmark-store-test/test.toml. Overwrite? (y/N): Overwriting file...\nBookmark file stored at HOME/.bookmark-store-test/test.toml\nBookmark added successfully as test\n"
        .replace("HOME", home.to_str().expect("Unable to convert HOME dir to str"));
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["add", "test", "https://test.local", "test"])
    .write_stdin("y\n")
    .assert()
    .success()
    .stdout(output_add_overwrite);

  // move
  let output_move = "Bookmark moved successfully from test to move/test\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["move", "test", "move/test"])
    .assert()
    .success()
    .stdout(output_move);

  // remove (removing the last entry in the bookmark-store-test removes it completely)
  let output_remove = "Bookmark removed successfully as move/test\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["remove", "move/test"])
    .assert()
    .success()
    .stdout(output_remove);
  // TODO: cleanup ~/.config/tempesta/tempesta.toml
}
