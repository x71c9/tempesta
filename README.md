# Tempesta

The fastest and lightest bookmark manager CLI writte in Rust.\
[Heavily inspired by [`pass`](https://www.passwordstore.org/)]

Bookmark management should be simple and follow [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy).

With Tempesta all bookmarks live in `~/.bookmark-store`, and `tempesta` provides
some intuitive commands for adding, updating, editing and open URLs.

It can also track all the changes using `git`.

## How to use it

#### Add a bookmark
```bash
tempesta add <url> <local-path>

tempesta add "http://google.com/" search-engines/google
```

#### Update a bookmark
```bash
tempesta update <local-path> <url>

tempesta update search-engines/google "https://google.com"
```

#### Edit a bookmark in your editor (it check $EDITOR variable)
```bash
tempesta edit <local-path>

tempesta edit search-engines/google
```

#### Open the URL in the browser
```bash
tempesta open <local-path>

tempesta open search-engines/google
```

#### Remove a bookmark
```bash
tempesta remove <local-path>

tempesta remove search-engines/google
```

## Shortcut

You can use the initial of the methods instead of their full name:

```bash
[a]dd     -- Add a new bookmark
[e]dit    -- Edit an existing bookmark
[o]pen    -- Open a bookmark
[r]emove  -- Remove a bookmark
[u]pdate  -- Update an existing bookmark
```

For example
```bash
tempesta o search-engines/google

tempesta r search-engines/google
```

## Install

### MacOS (Homebrew)

```bash
brew install x71c9/x71c9/tempesta
```
---
### Arch Linux (AUR)
```bash
yay -S tempesta
```
---
### Download binaries

Download the latest compatible binaries for your system and architecture:
[https://github.com/x71c9/tempesta/releases/latest](https://github.com/x71c9/tempesta/releases/latest)


---
### Build from source

#### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, ensure your environment is updated (or restart your 
terminal) so that the cargo and rustc commands are in your PATH.

```bash
git clone https://github.com/x71c9/tempesta
cd tempesta
cargo build --release

./target/release/tempesta
```

If you want to install the CLI tool so that it’s available in your PATH, 
you can use in the repo directory:
```bash
cargo install --path .
```
