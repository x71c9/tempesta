# Tempesta

The fastest and lightest bookmark manager CLI writte in Rust.\
[Heavily inspired by [`pass`](https://www.passwordstore.org/)]

Bookmark management should be simple and follow [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy).

With Tempesta all bookmarks live in `~/.bookmark-store`, and `tempesta` provides
some intuitive commands for adding, updating, editing and open URLs.

It can also track all the changes using `git`.

## Table of content

<details>
  <summary>How to use it</summary>

- [Init](#init)
- [Add a bookmark](#add-a-bookmark)
- [Edit a bookmark](#edit-a-bookmark)
- [List bookmarks](#list-bookmarks)
- [Move a bookmark](#move-a-bookmark)
- [Open the URL in the browser](#open-the-url-in-the-browser)
- [Remove a bookmark](#remove-a-bookmark)
- [Update a bookmark](#update-a-bookmark)

</details>

- [Shortcut](#shortcut)

<details>
  <summary>Install</summary>

- [MacOS (Homebrew)](#macos-homebrew)
- [Arch Linux (AUR)](#arch-linux-aur)
- [Download binaries](#download-binaries)
- [Build from source](#build-from-source)

</details>

<details>
  <summary>Alias</summary>

- [ZSH Alias](#zsh-alias)
- [Bash Alias](#bash-alias)

</details>

- [FZF](#fzf)
- [WOFI](#wofi)

## How to use it

#### Init

Before start using it, run:

```bash
tempesta init
```

This will prompt with questions about the set up.

#### Add a bookmark

```bash
tempesta add <local-path> <url>

tempesta add search-engines/google "http://google.com/"
```

#### Edit a bookmark in your editor (it check $EDITOR variable)

```bash
tempesta edit <local-path>

tempesta edit search-engines/google
```

#### List bookmarks

```bash
tempesta list <local-path>

tempesta list search-engines/
```

Without arguments it lists all the bookmarks
```bash
tempesta list
```

List as an parameter that can be passed with the flag `--divisor` that divides
the path name to the actual url:
```bash
tempesta list search-engines/ --divisor=" --- "

tempesta list search-engines/ --divisor " --- "
```

This prints:
```bash
search-engines/google --- https://google.com/
search-engines/duck --- https://duckduckgo.com/
```

#### Move a bookmark

```bash
tempesta move <local-path>

tempesta move search-engines/google query-engines/google
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

#### Update a bookmark

```bash
tempesta update <local-path> <url>

tempesta update search-engines/google "https://google.com"
```

## Shortcut

You can use the initial of the methods instead of their full identifier:

```bash
[a]dd     -- Add a new bookmark
[e]dit    -- Edit an existing bookmark
[o]pen    -- Open a bookmark
[m]ove    -- Move a bookmark
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

### Alias

#### ZSH Alias

Most likely you will alias the command with

```bash
alias t='tempesta'
```

In order to make completion in bash to work with alias you can add the following:

```bash
complete -o default -o nospace -F _tempesta t
```

where the final `t` is the name of the alias.

#### Bash Alias

Autocompletion do not work for bash alias but works when using functions,
therefore is recommended to use a function instead, for example:

```bash
t() {
  tempesta "$@"
}
complete -o default -o nospace -F _tempesta t
```

### FZF

If [`fzf`](https://github.com/junegunn/fzf) is installed on your system and the
method `open` is called without arguments:

```bash
tempesta open
```

it will start a fuzzy find of the bookmark.

A useful alias can be made for this:

```bash
alias to='tempesta o'
```

### Wofi

If [`wofi`](https://github.com/SimplyCEO/wofi) is installed on your system and
was selected during init as finder, `tempesta open` will create a graphical
fuzzy finder window instead of one in the terminal.
