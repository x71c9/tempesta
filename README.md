# Tempesta

The fastest and lightest bookmark manager CLI written in Rust.\
[Heavily inspired by [`pass`](https://www.passwordstore.org/)]

Bookmark management should be simple and follow [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy).

With Tempesta all bookmarks live in `~/.bookmark-store`, and `tempesta` provides
some intuitive commands for adding, updating, editing and open URLs.

It can also track all the changes using `git`.

## Table of content

How to use it:

- [Init](#init)
- [Add a bookmark](#add-a-bookmark)
- [Edit a bookmark](#edit-a-bookmark)
- [Get a bookmark](#get-a-bookmark)
- [Show configuration](#show-configuration)
- [List bookmarks](#list-bookmarks)
- [Move a bookmark](#move-a-bookmark)
- [Open the URL in the browser](#open-the-url-in-the-browser)
- [Remove a bookmark](#remove-a-bookmark)
- [Update a bookmark](#update-a-bookmark)
- [Shortcut](#shortcut)

Install

- [MacOS (Homebrew)](#macos-homebrew)
- [Arch Linux (AUR)](#arch-linux-aur)
- [Download binaries](#download-binaries)
- [Build from source](#build-from-source)
- [Shell completion](#shell-completion)

Alias

- [ZSH Alias](#zsh-alias)
- [Bash Alias](#bash-alias)

Combine

- [fzf](#fzf)
- [wofi](#wofi)

## How to use it

#### Init

Before start using it, run:

```bash
tempesta init
```

This will prompt with questions about the set up.

## Configuration

Tempesta's configuration file (`tempesta.toml`) is typically located at `~/.config/tempesta/tempesta.toml`.

You can specify a custom configuration file path using the `--config` or `-c` global flag. This flag can be placed anywhere in the command line:

```bash
tempesta --config /path/to/your/custom/tempesta.toml init
tempesta -c /path/to/another/config.toml list
```

If the `--config` flag is not provided, Tempesta will check for the `TEMPESTA_CONFIG` environment variable. If set, its value will be used as the configuration file path.

The `--config` flag takes precedence over the `TEMPESTA_CONFIG` environment variable. If neither is provided, Tempesta defaults to `~/.config/tempesta/tempesta.toml`.

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

#### Get a bookmark

It prints the URL of the bookmark

```bash
tempesta get <local-path>

tempesta get search-engines/google
```

#### Show configuration

It prints the current configuration

```bash
tempesta config
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

List has a parameter that can be passed with the flag `--divisor` that divides
the path name to the actual url:

```bash
tempesta list search-engines/ --divisor=" --- "
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
[c]onfig  -- Show the current configuration
[e]dit    -- Edit an existing bookmark
[g]et     -- Get an existing bookmark
[i]nit    -- Init the bookmark store
[l]ist    -- List bookmarks
[m]ove    -- Move a bookmark
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

### NixOS/Nix (NUR)

```bash
# Direct install (requires NUR)
nix-env -iA nur.repos.x71c9.tempesta -f '<nixpkgs>'

# Home Manager (requires NUR input/overlay)
home.packages = [ pkgs.nur.repos.x71c9.tempesta ];
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

### Shell completion

#### BASH

```bash
source <(tempesta completion bash)
# set up autocomplete in bash into the current shell,
# bash-completion package should be installed first.
echo "source <(tempesta completion bash)" >> ~/.bashrc
# add autocomplete permanently to your bash shell.
```

#### ZSH

```bash
source <(tempesta completion zsh)
# set up autocomplete in zsh into the current shell
echo '[[ $commands[tempesta] ]] && source <(tempesta completion zsh)' >> ~/.zshrc
# add autocomplete permanently to your zsh shell
```

#### FISH

```bash
echo 'tempesta completion fish | source' > ~/.config/fish/completions/tempesta.fish && source ~/.config/fish/completions/tempesta.fish
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

## fzf

If you are using `fzf` you can pipe the result of `tempesta list` to it:

```bash
tempesta list | fzf
```

And you can use it in combination with `tempesta open` in order to fuzzy find a
bookmark and open it in the browser

```bash
# with AWK
tempesta list | fzf | awk -F ' *:: *' '{print $1}' | xargs tempesta open
# with SED
tempesta list | fzf | sed 's/ *::.*//' | xargs tempesta open
```

An alias like this might be useful to open bookmarks:

```bash
alias tempo="tempesta list | fzf | sed 's/ *::.*//' | xargs tempesta open"
```

## wofi

If you are using `wofi` you can pipe the result of `tempesta list` to it:

```bash
tempesta list | wofi --dmenu --insensitive | xargs tempesta open
```

## rofi

If you are using `rofi` you can pipe the result of `tempesta list` to it:

```bash
tempesta list | rofi -dmenu -i -p "tempesta" | xargs tempesta open
```
