// use crate::add;
use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

use super::methods::config::Config;

fn test_setup(name: &str) -> String {
  let home_path = PathBuf::from(env::var("HOME").expect("HOME environment variable not set"));
  let home_str = home_path.to_str().expect("Cannot convert HOME to str");
  let test_config_file_path = format!("{}/.config/tempesta/test-tempesta-{}.toml", home_str, name);
  let test_bookmark_dir_path = format!("{}/.test-bookmark-store-{}", home_str, name);
  Command::cargo_bin("tempesta")
    .unwrap()
    .arg("init")
    .write_stdin(format!("{}\nno\n", &test_bookmark_dir_path));
  return test_config_file_path;
}


#[test]
fn tempesta_init(name: &str) {
  let home_string = test_env();
  let test_config_file_path =
    format!("{}/.config/tempesta/test-{}.toml", &home_string, name);
  let output = format!(
    concat!(
      "Where do you want to store the bookmarks? [~/.bookmark-store]: ",
      "Do you want to use Git for tracking bookmarks? (Y/n): ",
      "Tempesta initialized successfully: {}\n",
    ),
    &test_bookmark_store_dir_path
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .arg("init")
    .write_stdin(format!("{}\nno\n", &test_bookmark_store_dir_path))
    .assert()
    .success()
    .stdout(output);

  let config_exists = fs::exists(&test_config_file_path)
    .expect("The config file was not created");
  assert!(
    config_exists,
    "Config file was not created [{}]",
    &test_config_file_path
  );
  let config_string = fs::read_to_string(&test_config_file_path).expect(
    &format!("Canot read config file {}", &test_config_file_path),
  );
  let config: Config = toml::from_str(&config_string)
    .expect("Cannot parse config toml into Config");
  assert_eq!(&config.dir, &test_bookmark_store_dir_path);
  // TODO: check also other parameter
}

#[test]
fn tempesta_add() {
  tempesta_init("add");

  let home_string = test_env();
  let test_bookmark_store_dir_path =
    format!("{}/.bookmark-store-test", &home_string);
  let output_add = format!(
    concat!(
      "Bookmark file stored at {}/test.toml\n",
      "Bookmark added successfully as test\n"
    ),
    &test_bookmark_store_dir_path
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["add", "test", "https://test.local", "test"])
    .assert()
    .success()
    .stdout(output_add);
  // add (again but this time overwrite)
  let output_add_overwrite = format!(
    concat!(
      "Bookmark already exists at {}/test.toml. Overwrite? (y/N): ",
      "Overwriting file...\n",
      "Bookmark file stored at {}/test.toml\n",
      "Bookmark added successfully as test\n"
    ),
    &test_bookmark_store_dir_path, &test_bookmark_store_dir_path
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["add", "test", "https://test.local", "test"])
    .write_stdin("y\n")
    .assert()
    .success()
    .stdout(output_add_overwrite);
}

#[test]
fn tempesta_completion() {
  // TODO
}

#[test]
fn tempesta_config() {
  // TODO
}

#[test]
fn tempesta_list() {
  // TODO
}

#[test]
fn tempesta_move() {
  // move
  let output_move = "Bookmark moved successfully from test to move/test\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["move", "test", "move/test"])
    .assert()
    .success()
    .stdout(output_move);
}

#[test]
fn tempesta_open() {
  // TODO
}

#[test]
fn tempesta_remove() {
  // remove (removing the last entry in the bookmark-store-test removes it completely)
  let output_remove = "Bookmark removed successfully as move/test\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["remove", "move/test"])
    .assert()
    .success()
    .stdout(output_remove);
}

#[test]
fn tempesta_update() {
  // TODO
}

#[test]
fn cleanup() {
  let home = test_env();
  let home_str = home.to_str().expect("Unable to convert HOME dir to str");
  let test_config_file_path =
    format!("{}/.config/tempesta/test.toml", home_str);
  fs::remove_file(test_config_file_path).unwrap();
}
