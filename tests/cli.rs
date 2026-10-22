use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// ****************************************************************************
// Test environment
// ****************************************************************************

/// Isolated environment for one test: its own HOME, git configuration, config
/// file and bookmark store. Everything is removed on drop, also on failure.
struct TestEnv {
  root: PathBuf,
  config: PathBuf,
  store: PathBuf,
}

impl TestEnv {
  fn new(name: &str) -> Self {
    let root = std::env::temp_dir().join(format!(
      "tempesta-test-{}-{}",
      name,
      std::process::id()
    ));
    if root.exists() {
      fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir_all(root.join(".config/tempesta")).unwrap();
    TestEnv {
      config: root.join(".config/tempesta/tempesta.toml"),
      store: root.join("store"),
      root,
    }
  }

  /// Initialized environment without git
  fn init(name: &str) -> Self {
    let env = TestEnv::new(name);
    env
      .cmd()
      .arg("init")
      .write_stdin(format!("{}\nno\n", env.store_str()))
      .assert()
      .success();
    env
  }

  /// Initialized environment with git and an optional remote
  fn init_git(name: &str, remote: Option<&Path>) -> Self {
    let env = TestEnv::new(name);
    let stdin = match remote {
      Some(remote) => {
        format!("{}\ny\n{}\nmaster\n", env.store_str(), remote.display())
      }
      None => format!("{}\ny\n\n", env.store_str()),
    };
    env.cmd().arg("init").write_stdin(stdin).assert().success();
    env
  }

  /// The store path as written in the config, with a trailing slash
  fn store_str(&self) -> String {
    format!("{}/", self.store.display())
  }

  fn config_str(&self) -> String {
    self.config.display().to_string()
  }

  fn apply_env(&self, cmd: &mut std::process::Command) {
    cmd
      .env("HOME", &self.root)
      .env("XDG_CONFIG_HOME", self.root.join(".config"))
      .env("GIT_CONFIG_GLOBAL", "/dev/null")
      .env("GIT_CONFIG_NOSYSTEM", "1")
      .env("GIT_AUTHOR_NAME", "tempesta-test")
      .env("GIT_AUTHOR_EMAIL", "test@tempesta.local")
      .env("GIT_COMMITTER_NAME", "tempesta-test")
      .env("GIT_COMMITTER_EMAIL", "test@tempesta.local")
      .env_remove("TEMPESTA_CONFIG")
      .env_remove("BROWSER")
      .env_remove("EDITOR");
  }

  fn cmd(&self) -> Command {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_tempesta"));
    self.apply_env(&mut cmd);
    cmd.args(["--config", &self.config_str()]);
    Command::from_std(cmd)
  }

  fn run(&self, args: &[&str]) -> assert_cmd::assert::Assert {
    self.cmd().args(args).assert()
  }

  fn add(&self, path: &str, url: &str) {
    self.run(&["add", path, url]).success();
  }

  fn exists(&self, relative_path: &str) -> bool {
    self.store.join(relative_path).exists()
  }

  fn read(&self, relative_path: &str) -> String {
    fs::read_to_string(self.store.join(relative_path)).unwrap()
  }

  fn git(&self, dir: &Path, args: &[&str]) -> String {
    let mut cmd = std::process::Command::new("git");
    self.apply_env(&mut cmd);
    let output = cmd.args(args).current_dir(dir).output().unwrap();
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8(output.stdout).unwrap()
  }

  /// Commit subjects of the bookmark store, oldest first
  fn git_log(&self, dir: &Path) -> Vec<String> {
    let log = self.git(dir, &["log", "--reverse", "--format=%s"]);
    log.lines().map(String::from).collect()
  }

  #[cfg(unix)]
  fn script(&self, name: &str, body: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let path = self.root.join(name);
    fs::write(&path, format!("#!/usr/bin/env bash\n{}\n", body)).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    path
  }
}

impl Drop for TestEnv {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.root);
  }
}

fn stdout_of(assert: assert_cmd::assert::Assert) -> String {
  String::from_utf8(assert.get_output().stdout.clone()).unwrap()
}

// ****************************************************************************
// Global
// ****************************************************************************

#[test]
fn tempesta_usage() {
  let env = TestEnv::new("usage");
  env
    .run(&[])
    .failure()
    .stderr("Usage: tempesta <command> [options]\n");
  let stderr = String::from_utf8(
    env.run(&["unknown"]).failure().get_output().stderr.clone(),
  )
  .unwrap();
  assert!(stderr.starts_with("Unknown command: unknown\n"));
}

#[test]
fn tempesta_version() {
  let env = TestEnv::new("version");
  let expected = format!("Tempesta version: {}\n", env!("CARGO_PKG_VERSION"));
  for flag in ["--version", "-v"] {
    env.run(&[flag]).success().stdout(expected.clone());
  }
}

#[test]
fn tempesta_help() {
  let env = TestEnv::new("help");
  for flag in ["--help", "-h"] {
    let help = stdout_of(env.run(&[flag]).success());
    assert!(help.contains("USAGE:"));
    for command in [
      "add",
      "completion",
      "config",
      "edit",
      "get",
      "init",
      "list",
      "move",
      "native-host",
      "open",
      "remove",
      "update",
    ] {
      assert!(help.contains(command), "help does not mention {}", command);
    }
  }
}

#[test]
fn tempesta_aliases() {
  let env = TestEnv::init("aliases");
  env.run(&["a", "alias", "https://alias.local"]).success();
  env
    .run(&["g", "alias"])
    .success()
    .stdout("https://alias.local\n");
  for alias in ["l", "ls"] {
    env
      .run(&[alias])
      .success()
      .stdout("alias :: https://alias.local\n");
  }
  env.run(&["u", "alias", "https://updated.local"]).success();
  env.run(&["m", "alias", "moved"]).success();
  env.run(&["mv", "moved", "alias"]).success();
  env.run(&["c"]).success();
  env.run(&["r", "alias"]).success();
  env.add("alias", "https://alias.local");
  env.run(&["rm", "alias"]).success();
  assert!(!env.exists("alias.toml"));
}

// ****************************************************************************
// Commands
// ****************************************************************************

#[test]
fn tempesta_init() {
  let env = TestEnv::new("init");
  let output = format!(
    concat!(
      "Where do you want to store the bookmarks? [~/.bookmark-store]: ",
      "Do you want to use Git for tracking bookmarks? (Y/n): ",
      "Tempesta initialized successfully: {}\n",
    ),
    env.config_str()
  );
  env
    .cmd()
    .arg("init")
    .write_stdin(format!("{}\nno\n", env.store_str()))
    .assert()
    .success()
    .stdout(output);
  let config: toml::Value =
    toml::from_str(&fs::read_to_string(&env.config).unwrap()).unwrap();
  assert_eq!(config["dir"].as_str(), Some(env.store_str().as_str()));
  assert_eq!(config["git"].as_bool(), Some(false));
  assert_eq!(config["pull_before_push"].as_bool(), Some(false));
  assert_eq!(config["rebase_on_pull"].as_bool(), Some(false));
  assert!(config.get("remote").is_none());
}

#[test]
fn tempesta_init_default_store() {
  let env = TestEnv::new("init-default");
  env
    .cmd()
    .arg("init")
    .write_stdin("\nno\n")
    .assert()
    .success();
  let config: toml::Value =
    toml::from_str(&fs::read_to_string(&env.config).unwrap()).unwrap();
  let expected = env.root.join(".bookmark-store");
  assert_eq!(config["dir"].as_str(), expected.to_str());
}

#[test]
fn tempesta_config() {
  let env = TestEnv::init("config");
  let output = format!(
    concat!(
      "Git enabled:              false\n",
      "Remote: None\n",
      "Pull before push enabled: false\n",
      "Rebase on pull enabled:   false\n",
      "Bookmark store directory: {}\n",
    ),
    env.store_str()
  );
  env.run(&["config"]).success().stdout(output);
}

#[test]
fn tempesta_add() {
  let env = TestEnv::init("add");
  let output_add = format!(
    concat!(
      "Bookmark file stored at {}test.toml\n",
      "Bookmark added successfully as test\n",
    ),
    env.store_str()
  );
  env
    .run(&["add", "test", "https://test.local", "tag-a", "tag-b"])
    .success()
    .stdout(output_add);
  let bookmark: toml::Value = toml::from_str(&env.read("test.toml")).unwrap();
  assert_eq!(bookmark["url"].as_str(), Some("https://test.local"));
  assert_eq!(
    bookmark["tags"],
    toml::Value::Array(vec!["tag-a".into(), "tag-b".into()])
  );
  // add again, declining the overwrite
  env
    .cmd()
    .args(["add", "test", "https://other.local"])
    .write_stdin("n\n")
    .assert()
    .success()
    .stdout(format!(
      "Bookmark already exists at {}test.toml. Overwrite? (y/N): Operation cancelled.\n",
      env.store_str()
    ));
  assert!(env.read("test.toml").contains("https://test.local"));
  // add again, overwriting
  let output_add_overwrite = format!(
    concat!(
      "Bookmark already exists at {}test.toml. Overwrite? (y/N): ",
      "Overwriting file...\n",
      "Bookmark file stored at {}test.toml\n",
      "Bookmark added successfully as test\n"
    ),
    env.store_str(),
    env.store_str()
  );
  env
    .cmd()
    .args(["add", "test", "https://other.local"])
    .write_stdin("y\n")
    .assert()
    .success()
    .stdout(output_add_overwrite);
  assert!(env.read("test.toml").contains("https://other.local"));
  // invalid input
  env.run(&["add", "test"]).failure();
  env
    .run(&["add", "with space", "https://test.local"])
    .failure();
  env.run(&["add", "bad-url", "not-a-url"]).failure();
  assert!(!env.exists("bad-url.toml"));
}

#[test]
fn tempesta_get() {
  let env = TestEnv::init("get");
  env.add("dir/test", "https://get.local");
  env
    .run(&["get", "dir/test"])
    .success()
    .stdout("https://get.local\n");
  env.run(&["get", "dir/missing"]).failure();
  env.run(&["get"]).failure();
}

#[test]
fn tempesta_list() {
  let env = TestEnv::init("list");
  env.run(&["list"]).success().stderr(format!(
    "No .toml files found in {:?}\nNo bookmarks found.\n",
    PathBuf::from(env.store_str())
  ));
  env.add("test/test-list-b", "https://test-list.local.b");
  env.add("test/test-list-a", "https://test-list.local.a");
  env.add("other/test-list-c", "https://test-list.local.c");
  env.run(&["list"]).success().stdout(concat!(
    "other/test-list-c :: https://test-list.local.c\n",
    "test/test-list-a :: https://test-list.local.a\n",
    "test/test-list-b :: https://test-list.local.b\n",
  ));
  // sub directory
  env
    .run(&["list", "other"])
    .success()
    .stdout("other/test-list-c :: https://test-list.local.c\n");
  // custom divisor, both forms
  for args in [
    vec!["list", "other", "--divisor", " | "],
    vec!["list", "other", "--divisor= | "],
  ] {
    env
      .run(&args)
      .success()
      .stdout("other/test-list-c | https://test-list.local.c\n");
  }
}

#[cfg(unix)]
#[test]
fn tempesta_edit() {
  let env = TestEnv::init("edit");
  env.add("test", "https://edit.local");
  // an editor that rewrites the bookmark
  let editor = env.script(
    "editor",
    r#"printf 'url = "https://edited.local"\ntags = []\n' > "$1""#,
  );
  env
    .cmd()
    .env("EDITOR", &editor)
    .args(["edit", "test"])
    .assert()
    .success()
    .stdout("Bookmark edited successfully as test\n");
  env
    .run(&["get", "test"])
    .success()
    .stdout("https://edited.local\n");
  // an editor that changes nothing
  let noop = env.script("noop-editor", "exit 0");
  env
    .cmd()
    .env("EDITOR", &noop)
    .args(["edit", "test"])
    .assert()
    .success()
    .stdout("No changes made.\n");
  // an editor that fails
  let failing = env.script("failing-editor", "exit 1");
  env
    .cmd()
    .env("EDITOR", &failing)
    .args(["edit", "test"])
    .assert()
    .failure()
    .stderr("Failed to edit bookmark file.\n");
  env.run(&["edit", "missing"]).failure();
}

#[cfg(target_os = "linux")]
#[test]
fn tempesta_open() {
  let env = TestEnv::init("open");
  env.add("test", "https://open.local");
  let opened = env.root.join("opened");
  let browser = env.script(
    "browser",
    &format!(r#"printf '%s' "$1" > "{}""#, opened.display()),
  );
  env
    .cmd()
    .env("BROWSER", &browser)
    .args(["open", "test"])
    .assert()
    .success();
  // The browser is spawned in the background
  let deadline = Instant::now() + Duration::from_secs(10);
  while !opened.exists() && Instant::now() < deadline {
    std::thread::sleep(Duration::from_millis(20));
  }
  assert_eq!(fs::read_to_string(&opened).unwrap(), "https://open.local/");
  env
    .cmd()
    .env("BROWSER", &browser)
    .args(["open", "missing"])
    .assert()
    .failure();
}

#[test]
fn tempesta_move() {
  let env = TestEnv::init("move");
  env.add("test", "https://test.local");
  // Add a bookmark at the destination to trigger the overwrite prompt
  env.add("move/test", "https://test.local/moved");
  let output_move = format!(
    concat!(
      "A bookmark already exists at {}move/test.toml. Overwrite? [Y/n]: ",
      "Bookmark moved successfully from test to move/test\n"
    ),
    env.store_str()
  );
  env
    .cmd()
    .args(["move", "test", "move/test"])
    .write_stdin("y\n")
    .assert()
    .success()
    .stdout(output_move);
  assert!(!env.exists("test.toml"));
  assert!(env.read("move/test.toml").contains("https://test.local\""));
  // declining the overwrite keeps both bookmarks
  env.add("test", "https://test.local/other");
  env
    .cmd()
    .args(["move", "test", "move/test"])
    .write_stdin("n\n")
    .assert()
    .success();
  assert!(env.read("test.toml").contains("https://test.local/other"));
  // trailing slash keeps the file name
  env.run(&["move", "test", "into/"]).success();
  assert!(env.exists("into/test.toml"));
  // the emptied source directory is removed
  env.run(&["move", "into/test", "test"]).success();
  assert!(!env.exists("into"));
  env.run(&["move", "missing", "other"]).failure();
  env.run(&["move", "test"]).failure();
}

#[test]
fn tempesta_move_directory() {
  let env = TestEnv::init("move-directory");
  let move_dir = |from: &str, to: &str, stdin: &str| {
    env
      .cmd()
      .args(["move", from, to])
      .write_stdin(stdin)
      .assert()
  };
  env.add("work/github", "https://github.com");
  env.add("work/sub/gitlab", "https://gitlab.com");
  env.add("archive/old", "https://old.local");

  // rename
  move_dir("work", "personal", "")
    .success()
    .stdout("Directory moved successfully from work to personal\n");
  assert!(env.exists("personal/github.toml"));
  assert!(env.exists("personal/sub/gitlab.toml"));
  assert!(!env.exists("work"));

  // trailing slash moves into the destination
  move_dir("personal", "archive/", "").success();
  assert!(env.exists("archive/personal/github.toml"));
  assert!(env.exists("archive/personal/sub/gitlab.toml"));
  assert!(env.exists("archive/old.toml"));
  assert!(!env.exists("personal"));

  // existing destination is merged, overwriting after confirmation
  env.add("dest/personal/github", "https://dest.local");
  move_dir("archive/personal", "dest/", "y\n").success();
  assert!(env
    .read("dest/personal/github.toml")
    .contains("https://github.com"));
  assert!(env.exists("dest/personal/sub/gitlab.toml"));
  assert!(!env.exists("archive/personal"));
  assert!(env.exists("archive/old.toml"));

  // declined overwrite keeps the source bookmark in place
  env.add("keep/personal/github", "https://keep.local");
  move_dir("keep/personal", "dest/", "n\n").success();
  assert!(env
    .read("dest/personal/github.toml")
    .contains("https://github.com"));
  assert!(env
    .read("keep/personal/github.toml")
    .contains("https://keep.local"));

  move_dir("dest", "dest/inner", "")
    .failure()
    .stderr("Cannot move a directory into itself\n");
}

#[test]
fn tempesta_remove() {
  let env = TestEnv::init("remove");
  env.add("move/test", "https://test.local");
  env.add("keep", "https://keep.local");
  // removing the last bookmark of a directory removes the directory too
  env
    .run(&["remove", "move/test"])
    .success()
    .stdout("Bookmark removed successfully as move/test\n");
  assert!(!env.exists("move"));
  assert!(env.exists("keep.toml"));
  // directory, declined
  env.add("dir/a", "https://a.local");
  env.add("dir/sub/b", "https://b.local");
  env
    .cmd()
    .args(["remove", "dir"])
    .write_stdin("n\n")
    .assert()
    .success()
    .stdout(concat!(
      "Bookmark not found as a file, but 'dir' is a directory. ",
      "Do you want to delete it and all its bookmarks? [Y/n] ",
      "Operation canceled.\n"
    ));
  assert!(env.exists("dir/sub/b.toml"));
  // directory, confirmed
  env
    .cmd()
    .args(["remove", "dir"])
    .write_stdin("y\n")
    .assert()
    .success();
  assert!(!env.exists("dir"));
  // missing
  env.run(&["remove", "missing"]).failure();
  env.run(&["remove"]).failure();
}

#[test]
fn tempesta_update() {
  let env = TestEnv::init("update");
  let bookmark_path = "test/test-update";
  env
    .run(&["add", bookmark_path, "https://test-update.local", "old-tag"])
    .success();
  let output_update_bookmark = format!(
    concat!(
      "Bookmark file stored at {}{}.toml\n",
      "Bookmark updated successfully as {}\n"
    ),
    env.store_str(),
    bookmark_path,
    bookmark_path
  );
  let new_url = "http://test-update.new";
  env
    .run(&["update", bookmark_path, new_url, "new-tag"])
    .success()
    .stdout(output_update_bookmark);
  env
    .run(&["get", bookmark_path])
    .success()
    .stdout(format!("{}\n", new_url));
  let bookmark: toml::Value =
    toml::from_str(&env.read("test/test-update.toml")).unwrap();
  assert_eq!(bookmark["tags"], toml::Value::Array(vec!["new-tag".into()]));
  env.run(&["update", "missing", new_url]).failure();
  env.run(&["update", bookmark_path, "not-a-url"]).failure();
}

#[test]
fn tempesta_completion() {
  let env = TestEnv::new("completion");
  // Each script must register the binary and its aliases (t, tmps)
  let cases = [
    ("bash", "complete -F _tempesta tempesta t tmps"),
    ("zsh", "compdef _tempesta tempesta t tmps"),
    ("fish", "for bin in tempesta t tmps"),
  ];
  for (shell, expected) in cases {
    let script = stdout_of(env.run(&["completion", shell]).success());
    assert!(
      script.contains(expected),
      "{} completion script does not contain `{}`",
      shell,
      expected
    );
    // Without an argument the shell is detected from $SHELL
    let detected = stdout_of(
      env
        .cmd()
        .env("SHELL", format!("/usr/bin/{}", shell))
        .arg("completion")
        .assert()
        .success(),
    );
    assert_eq!(detected, script);
  }
  // Unknown shells fall back to bash
  let fallback = stdout_of(
    env
      .cmd()
      .env("SHELL", "/bin/unknown")
      .args(["completion", "unknown"])
      .assert()
      .success(),
  );
  assert!(fallback.contains("complete -F _tempesta tempesta t tmps"));
}

// ****************************************************************************
// Git
// ****************************************************************************

#[cfg(unix)]
#[test]
fn tempesta_git() {
  let env = TestEnv::init_git("git", None);
  assert!(env.store.join(".git").is_dir());
  env.add("a", "https://a.local");
  env.run(&["update", "a", "https://a2.local"]).success();
  env.run(&["move", "a", "dir/a"]).success();
  env.add("dir/b", "https://b.local");
  env.run(&["move", "dir", "other"]).success();
  let editor = env.script(
    "editor",
    r#"printf 'url = "https://edited.local"\ntags = []\n' > "$1""#,
  );
  env
    .cmd()
    .env("EDITOR", &editor)
    .args(["edit", "other/a"])
    .assert()
    .success();
  env.run(&["remove", "other/a"]).success();
  env
    .cmd()
    .args(["remove", "other"])
    .write_stdin("y\n")
    .assert()
    .success();
  let log = env.git_log(&env.store);
  let expected = [
    "Add bookmark a".to_string(),
    "Update bookmark a".to_string(),
    "Move bookmark from a to dir/a".to_string(),
    "Add bookmark dir/b".to_string(),
    "Move directory from dir to other".to_string(),
    "Edit bookmark other/a".to_string(),
    "Remove bookmark other/a".to_string(),
    "Removed directory other and all bookmarks".to_string(),
  ];
  assert_eq!(log, expected);
  // nothing left uncommitted
  assert_eq!(env.git(&env.store, &["status", "--porcelain"]), "");
}

#[test]
fn tempesta_git_remote() {
  let remote_env = TestEnv::new("git-remote-origin");
  let remote = remote_env.root.join("origin.git");
  remote_env.git(&remote_env.root, &["init", "--bare", "-q", "origin.git"]);

  let env = TestEnv::init_git("git-remote", Some(&remote));
  let config: toml::Value =
    toml::from_str(&fs::read_to_string(&env.config).unwrap()).unwrap();
  assert_eq!(config["remote"].as_str(), remote.to_str());
  env.add("a", "https://a.local");
  assert_eq!(env.git_log(&remote), ["Add bookmark a"]);

  // pull before push
  let content = fs::read_to_string(&env.config)
    .unwrap()
    .replace("pull_before_push = false", "pull_before_push = true");
  fs::write(&env.config, content).unwrap();
  env.add("b", "https://b.local");
  assert_eq!(env.git_log(&remote), ["Add bookmark a", "Add bookmark b"]);
}

// ****************************************************************************
// Native messaging host
// ****************************************************************************

fn native_message(json: &str) -> Vec<u8> {
  let mut message = (json.len() as u32).to_ne_bytes().to_vec();
  message.extend_from_slice(json.as_bytes());
  message
}

fn native_request(
  env: &TestEnv,
  args: &[&str],
  json: &str,
) -> serde_json::Value {
  let output = env
    .cmd()
    .args(args)
    .write_stdin(native_message(json))
    .assert()
    .success()
    .get_output()
    .stdout
    .clone();
  let length = u32::from_ne_bytes(output[..4].try_into().unwrap()) as usize;
  assert_eq!(output.len(), 4 + length, "stdout must be a single message");
  serde_json::from_slice(&output[4..]).expect("response is json")
}

#[test]
fn tempesta_native_host() {
  let env = TestEnv::init("native-host");
  let host = ["native-host"];
  env
    .run(&["add", "test/tagged", "https://test-tagged.local", "keep-me"])
    .success();
  // add
  let response = native_request(
    &env,
    &host,
    r#"{"action":"add","path":"test/new","url":"https://test-new.local"}"#,
  );
  assert_eq!(response, serde_json::json!({ "ok": true }));
  // add on an existing path without overwrite
  let response = native_request(
    &env,
    &host,
    r#"{"action":"add","path":"test/tagged","url":"https://other.local"}"#,
  );
  assert_eq!(
    response,
    serde_json::json!({ "ok": false, "code": "exists" })
  );
  // overwrite keeps the existing tags
  let response = native_request(
    &env,
    &host,
    r#"{"action":"add","path":"test/tagged","url":"https://other.local","overwrite":true}"#,
  );
  assert_eq!(response, serde_json::json!({ "ok": true }));
  // list, launched the way Chromium and Firefox do
  let expected = serde_json::json!({
    "ok": true,
    "bookmarks": [
      { "path": "test/new", "url": "https://test-new.local", "tags": [] },
      { "path": "test/tagged", "url": "https://other.local", "tags": ["keep-me"] },
    ],
  });
  for args in [
    vec!["chrome-extension://mmghkkaaciolnpicbbmdhieeginfpkmf/"],
    vec![
      "/hosts/com.x71c9.tempesta.json",
      "tempesta-browser-extension@x71c9.com",
    ],
  ] {
    let response = native_request(&env, &args, r#"{"action":"list"}"#);
    assert_eq!(response, expected);
  }
  // invalid requests become error responses
  for json in [
    r#"{"action":"add","path":"../escape","url":"https://test.local"}"#,
    r#"{"action":"add","path":"with space","url":"https://test.local"}"#,
    r#"{"action":"add","path":"test/bad-url","url":"not-a-url"}"#,
    r#"{"action":"unknown"}"#,
  ] {
    let response = native_request(&env, &host, json);
    assert_eq!(response["ok"], false, "{}", json);
    assert_eq!(response["code"], "error", "{}", json);
  }
  assert!(!env.root.join("escape.toml").exists());
}

#[test]
fn tempesta_native_host_install_print() {
  let env = TestEnv::new("native-host-install");
  let output = env
    .run(&[
      "native-host",
      "install",
      "--browser",
      "firefox",
      "--path",
      "/bin/tempesta",
      "--print",
    ])
    .success()
    .get_output()
    .stdout
    .clone();
  let manifest: serde_json::Value =
    serde_json::from_slice(&output).expect("manifest is json");
  assert_eq!(manifest["name"], "com.x71c9.tempesta");
  assert_eq!(manifest["path"], "/bin/tempesta");
  assert_eq!(manifest["type"], "stdio");
  assert_eq!(
    manifest["allowed_extensions"],
    serde_json::json!(["tempesta-browser-extension@x71c9.com"])
  );
}
