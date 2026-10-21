// ****************************************************************************
// Browser native messaging host (used by tempesta-browser-extension)
//
// Messages are JSON documents prefixed by their length as a 32-bit unsigned
// integer in native byte order, both on stdin and stdout.
// ****************************************************************************

use super::common::{self, PanicOnError};
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::Ordering;

pub const HOST_NAME: &str = "com.x71c9.tempesta";
const HOST_DESCRIPTION: &str = "Tempesta bookmark store";
const FIREFOX_EXTENSION_ID: &str = "tempesta-browser-extension@x71c9.com";
const CHROMIUM_EXTENSION_IDS: &[&str] = &["mmghkkaaciolnpicbbmdhieeginfpkmf"];

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
enum Request {
  List,
  Add {
    path: String,
    url: String,
    #[serde(default)]
    overwrite: bool,
  },
  Version,
}

// Browsers launch the host with their own arguments:
// Chromium passes the caller origin, Firefox the manifest path and the
// extension id
pub fn is_browser_invocation(args: &[String]) -> bool {
  let manifest_file = format!("{}.json", HOST_NAME);
  match args.get(1) {
    Some(arg) => {
      arg.starts_with("chrome-extension://")
        || arg.ends_with(&manifest_file)
        || args.get(2).is_some_and(|id| id == FIREFOX_EXTENSION_ID)
    }
    None => false,
  }
}

pub fn run(args: Vec<String>) {
  match args.get(2).map(String::as_str) {
    None => serve(),
    Some("install") => install(&args[3..]),
    Some(other) => {
      eprintln!("Unknown native-host command: {}", other);
      eprintln!(
        "Usage: tempesta native-host [install [--browser <name>] [--path <bin>] [--print]]"
      );
      std::process::exit(1);
    }
  }
}

// ****************************************************************************
// Serve requests until the browser closes stdin
// ****************************************************************************
pub fn serve() {
  common::NATIVE_HOST.store(true, Ordering::Relaxed);
  let mut stdin = io::stdin().lock();
  let mut stdout = io::stdout().lock();
  while let Some(message) = read_message(&mut stdin) {
    let response = handle(&message);
    write_message(&mut stdout, &response);
  }
}

fn read_message(reader: &mut impl Read) -> Option<Vec<u8>> {
  let mut length = [0u8; 4];
  if reader.read_exact(&mut length).is_err() {
    return None;
  }
  let mut message = vec![0u8; u32::from_ne_bytes(length) as usize];
  reader
    .read_exact(&mut message)
    .panic_on_error("Failed to read native message");
  Some(message)
}

fn write_message(writer: &mut impl Write, value: &Value) {
  let message =
    serde_json::to_vec(value).panic_on_error("Failed to serialize response");
  let length = u32::try_from(message.len())
    .panic_on_error("Native message too long")
    .to_ne_bytes();
  writer
    .write_all(&length)
    .and_then(|_| writer.write_all(&message))
    .and_then(|_| writer.flush())
    .panic_on_error("Failed to write native message");
}

// Tempesta reports errors by panicking: turn them into error responses
fn handle(message: &[u8]) -> Value {
  let result = panic::catch_unwind(AssertUnwindSafe(|| {
    match serde_json::from_slice::<Request>(message) {
      Ok(request) => dispatch(request),
      Err(e) => error(&format!("Invalid request: {}", e)),
    }
  }));
  result.unwrap_or_else(|payload| {
    let message = payload
      .downcast_ref::<String>()
      .map(String::as_str)
      .or_else(|| payload.downcast_ref::<&str>().copied())
      .unwrap_or("Unknown error");
    error(message)
  })
}

fn error(message: &str) -> Value {
  json!({ "ok": false, "code": "error", "message": message })
}

fn dispatch(request: Request) -> Value {
  match request {
    Request::List => list(),
    Request::Add {
      path,
      url,
      overwrite,
    } => add(&path, &url, overwrite),
    Request::Version => {
      json!({ "ok": true, "version": env!("CARGO_PKG_VERSION") })
    }
  }
}

fn list() -> Value {
  let bookmarks: Vec<Value> = common::get_toml_bookmark_files(None)
    .into_iter()
    .filter_map(|path| {
      let toml_file_path = common::get_bookmark_file_path(&path);
      let (url, tags) = read_bookmark(&toml_file_path)?;
      Some(json!({ "path": path, "url": url, "tags": tags }))
    })
    .collect();
  json!({ "ok": true, "bookmarks": bookmarks })
}

fn add(relative_path: &str, url: &str, overwrite: bool) -> Value {
  let relative_path = relative_path.to_string();
  common::validate_path(&relative_path);
  validate_relative(&relative_path);
  common::validate_url(url);
  let toml_file_path = common::get_bookmark_file_path(&relative_path);
  let existing = read_bookmark(&toml_file_path);
  if existing.is_some() && !overwrite {
    return json!({ "ok": false, "code": "exists" });
  }
  let action = if existing.is_some() { "Update" } else { "Add" };
  // Overwriting keeps the tags of the existing bookmark
  let tags = existing.map(|(_, tags)| tags).unwrap_or_default();
  common::store_bookmark(&toml_file_path, url, &tags);
  common::git_commit(&format!("{} bookmark {}", action, relative_path));
  json!({ "ok": true })
}

// Paths come from the browser: keep them inside the bookmark store
fn validate_relative(relative_path: &str) {
  let inside_store = Path::new(relative_path)
    .components()
    .all(|c| matches!(c, Component::Normal(_)));
  if !inside_store || relative_path.ends_with('/') {
    panic!("Invalid path. It must be relative to the bookmark store.");
  }
}

// Lenient reader: bookmarks without tags are still listed
fn read_bookmark(toml_file_path: &Path) -> Option<(String, Vec<String>)> {
  let content = fs::read_to_string(toml_file_path).ok()?;
  let toml: toml::Value = toml::from_str(&content).ok()?;
  let url = toml.get("url")?.as_str()?.to_string();
  let tags = toml
    .get("tags")
    .and_then(|t| t.as_array())
    .map(|tags| {
      tags
        .iter()
        .filter_map(|t| t.as_str().map(String::from))
        .collect()
    })
    .unwrap_or_default();
  Some((url, tags))
}

// ****************************************************************************
// Register the host manifest for the browsers
// ****************************************************************************

#[derive(Clone, Copy, PartialEq)]
enum Browser {
  Firefox,
  Chromium,
  Chrome,
  Brave,
  Edge,
  Vivaldi,
  Opera,
}

const BROWSERS: [Browser; 7] = [
  Browser::Firefox,
  Browser::Chromium,
  Browser::Chrome,
  Browser::Brave,
  Browser::Edge,
  Browser::Vivaldi,
  Browser::Opera,
];

impl Browser {
  fn from_name(name: &str) -> Option<Self> {
    match name {
      "firefox" => Some(Browser::Firefox),
      "chromium" => Some(Browser::Chromium),
      "chrome" => Some(Browser::Chrome),
      "brave" => Some(Browser::Brave),
      "edge" => Some(Browser::Edge),
      "vivaldi" => Some(Browser::Vivaldi),
      "opera" => Some(Browser::Opera),
      _ => None,
    }
  }
  fn name(&self) -> &'static str {
    match self {
      Browser::Firefox => "firefox",
      Browser::Chromium => "chromium",
      Browser::Chrome => "chrome",
      Browser::Brave => "brave",
      Browser::Edge => "edge",
      Browser::Vivaldi => "vivaldi",
      Browser::Opera => "opera",
    }
  }
  // Browser profile directory, relative to home
  #[cfg(target_os = "macos")]
  fn profile_dir(&self) -> &'static str {
    match self {
      Browser::Firefox => "Library/Application Support/Mozilla",
      Browser::Chromium => "Library/Application Support/Chromium",
      Browser::Chrome => "Library/Application Support/Google/Chrome",
      Browser::Brave => {
        "Library/Application Support/BraveSoftware/Brave-Browser"
      }
      Browser::Edge => "Library/Application Support/Microsoft Edge",
      Browser::Vivaldi => "Library/Application Support/Vivaldi",
      Browser::Opera => "Library/Application Support/com.operasoftware.Opera",
    }
  }
  #[cfg(not(target_os = "macos"))]
  fn profile_dir(&self) -> &'static str {
    match self {
      Browser::Firefox => ".mozilla",
      Browser::Chromium => ".config/chromium",
      Browser::Chrome => ".config/google-chrome",
      Browser::Brave => ".config/BraveSoftware/Brave-Browser",
      Browser::Edge => ".config/microsoft-edge",
      Browser::Vivaldi => ".config/vivaldi",
      Browser::Opera => ".config/opera",
    }
  }
  fn hosts_dir(&self) -> PathBuf {
    let home = dirs::home_dir().panic_on_error("Could not find home directory");
    let hosts = match (self, cfg!(target_os = "macos")) {
      (Browser::Firefox, false) => "native-messaging-hosts",
      _ => "NativeMessagingHosts",
    };
    home.join(self.profile_dir()).join(hosts)
  }
  fn profile_exists(&self) -> bool {
    self.hosts_dir().parent().is_some_and(Path::exists)
  }
  fn manifest(&self, bin_path: &Path) -> Value {
    let mut manifest = json!({
      "name": HOST_NAME,
      "description": HOST_DESCRIPTION,
      "path": bin_path,
      "type": "stdio",
    });
    if *self == Browser::Firefox {
      manifest["allowed_extensions"] = json!([FIREFOX_EXTENSION_ID]);
    } else {
      let origins: Vec<String> = CHROMIUM_EXTENSION_IDS
        .iter()
        .map(|id| format!("chrome-extension://{}/", id))
        .collect();
      manifest["allowed_origins"] = json!(origins);
    }
    manifest
  }
}

fn install(args: &[String]) {
  let mut browser: Option<Browser> = None;
  let mut bin_path: Option<PathBuf> = None;
  let mut print = false;
  let mut args_iter = args.iter();
  while let Some(arg) = args_iter.next() {
    match arg.as_str() {
      "--browser" => {
        let name = args_iter.next().map(String::as_str).unwrap_or("");
        if name != "all" {
          browser = Some(Browser::from_name(name).unwrap_or_else(|| {
            eprintln!(
              "Unknown browser {:?}: use firefox, chromium, chrome, brave, edge, vivaldi, opera or all",
              name
            );
            std::process::exit(1);
          }));
        }
      }
      "--path" => bin_path = args_iter.next().map(PathBuf::from),
      "--print" => print = true,
      other => {
        eprintln!("Unknown option: {}", other);
        std::process::exit(1);
      }
    }
  }
  let bin_path = bin_path.unwrap_or_else(find_binary);
  if print {
    let Some(browser) = browser else {
      eprintln!("--print requires --browser <name>");
      std::process::exit(1);
    };
    let manifest = serde_json::to_string_pretty(&browser.manifest(&bin_path))
      .panic_on_error("Failed to serialize manifest");
    println!("{}", manifest);
    return;
  }
  // Without an explicit browser, register only the installed ones
  let browsers: Vec<Browser> = match browser {
    Some(b) => vec![b],
    None => BROWSERS
      .into_iter()
      .filter(Browser::profile_exists)
      .collect(),
  };
  if browsers.is_empty() {
    eprintln!("No supported browser found, use --browser <name>");
    std::process::exit(1);
  }
  for browser in browsers {
    let hosts_dir = browser.hosts_dir();
    fs::create_dir_all(&hosts_dir)
      .panic_on_error("Failed to create native messaging hosts directory");
    let manifest_path = hosts_dir.join(format!("{}.json", HOST_NAME));
    let manifest = serde_json::to_string_pretty(&browser.manifest(&bin_path))
      .panic_on_error("Failed to serialize manifest");
    fs::write(&manifest_path, manifest + "\n")
      .panic_on_error("Failed to write native messaging host manifest");
    println!(
      "Native messaging host installed for {}: {}",
      browser.name(),
      manifest_path.display()
    );
  }
}

// The first `tempesta` in PATH is preferred over the resolved executable:
// on Nix it is a stable profile link instead of a versioned store path
fn find_binary() -> PathBuf {
  let from_path = env::var_os("PATH").and_then(|paths| {
    env::split_paths(&paths)
      .map(|dir| dir.join("tempesta"))
      .find(|candidate| candidate.is_file() && candidate.is_absolute())
  });
  from_path.unwrap_or_else(|| {
    env::current_exe().panic_on_error("Cannot find the tempesta executable")
  })
}
