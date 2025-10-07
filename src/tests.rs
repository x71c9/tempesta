use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

use super::methods::config::Config;

fn get_home() -> String {
  let home_path =
    PathBuf::from(env::var("HOME").expect("HOME environment variable not set"));
  let home_string = home_path.to_str().expect("Cannot convert HOME to str");
  return home_string.to_string();
}
fn get_test_config_file_path(home_str: &str, name: &str) -> String {
  let test_config_file_path =
    format!("{}/.config/tempesta/test-tempesta-{}.toml", &home_str, name);
  return test_config_file_path;
}
fn get_test_bookmark_dir_path(home_str: &str, name: &str) -> String {
  let test_bookmark_dir_path =
    format!("{}/.test-bookmark-store-{}/", &home_str, name);
  return test_bookmark_dir_path;
}
fn test_setup(name: &str) -> (String, String) {
  let home_str = get_home();
  let test_config_file_path = get_test_config_file_path(&home_str, name);
  let test_bookmark_dir_path = get_test_bookmark_dir_path(&home_str, name);
  Command::cargo_bin("tempesta")
    .unwrap()
    .arg("init")
    .args(["--config", &test_config_file_path])
    .write_stdin(format!("{}\nno\n", &test_bookmark_dir_path))
    .assert()
    .success();
  return (test_config_file_path, test_bookmark_dir_path);
}
fn test_cleanup(name: &str) {
  let home_str = get_home();
  let test_config_file_path = get_test_config_file_path(&home_str, name);
  let test_bookmark_dir_path = get_test_bookmark_dir_path(&home_str, name);
  // Check if the config file exists before attempting to remove it
  if fs::metadata(&test_config_file_path).is_ok() {
    fs::remove_file(test_config_file_path).unwrap();
  }
  // Check if the bookmark directory exists before attempting to remove it
  if fs::metadata(&test_bookmark_dir_path).is_ok() {
    fs::remove_dir_all(&test_bookmark_dir_path).unwrap();
  }
}

#[test]
fn tempesta_init() {
  let name = "init";
  let home_str = get_home();
  let test_config_file_path =
    format!("{}/.config/tempesta/test-tempesta-{}.toml", &home_str, name);
  let test_bookmark_dir_path =
    format!("{}/.test-bookmark-store-{}/", &home_str, name);
  let output = format!(
    concat!(
      "Where do you want to store the bookmarks? [~/.bookmark-store]: ",
      "Do you want to use Git for tracking bookmarks? (Y/n): ",
      "Tempesta initialized successfully: {}\n",
    ),
    &test_config_file_path
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .arg("init")
    .args(["--config", &test_config_file_path])
    .write_stdin(format!("{}\nno\n", &test_bookmark_dir_path))
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
  assert_eq!(&config.dir, &test_bookmark_dir_path);
  // TODO: check also other parameter
  test_cleanup(name);
}

#[test]
fn tempesta_add() {
  let name = "add";
  let (test_config_file_path, test_bookmark_dir_path) = test_setup(name);
  let bookmark_file_path = format!("{}/test.toml", &test_bookmark_dir_path);
  if fs::metadata(&bookmark_file_path).is_ok() {
    fs::remove_file(&bookmark_file_path).unwrap();
  }
  let output_add = format!(
    concat!(
      "Bookmark file stored at {}test.toml\n",
      "Bookmark added successfully as test\n",
    ),
    &test_bookmark_dir_path
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "add",
      "test",
      "https://test.local",
      "test",
      "--config",
      &test_config_file_path,
    ])
    .assert()
    .success()
    .stdout(output_add);
  // add (again but this time overwrite)
  let output_add_overwrite = format!(
    concat!(
      "Bookmark already exists at {}test.toml. Overwrite? (y/N): ",
      "Overwriting file...\n",
      "Bookmark file stored at {}test.toml\n",
      "Bookmark added successfully as test\n"
    ),
    &test_bookmark_dir_path,
    &test_bookmark_dir_path
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "add",
      "test",
      "https://test.local",
      "test",
      "--config",
      &test_config_file_path,
    ])
    .write_stdin("y\n")
    .assert()
    .success()
    .stdout(output_add_overwrite);
  test_cleanup(name);
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
  let name = "move";
  let (test_config_file_path, test_bookmark_dir_path) = test_setup(name);
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "add",
      "test",
      "https://test.local",
      "test",
      "--config",
      &test_config_file_path,
    ])
    .assert()
    .success();
  // Add a bookmark at the destination to trigger the overwrite prompt
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "add",
      "move/test",
      "https://test.local/moved",
      "test",
      "--config",
      &test_config_file_path,
    ])
    .assert()
    .success();
  // move
  let output_move = format!(
    concat!(
      "A bookmark already exists at {}move/test.toml. Overwrite? [Y/n]: ",
      "Bookmark moved successfully from test to move/test\n"
    ),
    &test_bookmark_dir_path
  );
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "move",
      "test",
      "move/test",
      "--config",
      &test_config_file_path,
    ])
    .write_stdin("y\n")
    .assert()
    .success()
    .stdout(output_move);
  test_cleanup(name);
}

#[test]
fn tempesta_open() {
  // TODO
}

#[test]
fn tempesta_remove() {
  let name = "remove";
  let (test_config_file_path, _test_bookmakr_dir_path) = test_setup(name); // Fixed unused variable
  Command::cargo_bin("tempesta")
    .unwrap()
    .args([
      "add",
      "move/test",
      "https://test.local",
      "test",
      "--config",
      &test_config_file_path,
    ])
    .assert()
    .success();
  // remove (removing the last entry in the bookmark-store-test removes it completely)
  let output_remove = "Bookmark removed successfully as move/test\n";
  Command::cargo_bin("tempesta")
    .unwrap()
    .args(["remove", "move/test", "--config", &test_config_file_path])
    .assert()
    .success()
    .stdout(output_remove);
  test_cleanup(name);
}

#[test]
fn tempesta_update() {
  // TODO
}
