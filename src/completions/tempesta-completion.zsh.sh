#compdef tempesta
#autoload

_tempesta () {
  local cmd
  if (( CURRENT > 2 )); then
    cmd=${words[2]}
    curcontext="${curcontext%:*:*}:tempesta-$cmd"
    (( CURRENT-- ))
    shift words

    case "${cmd}" in
      add|a|edit|remove|open|update|e|r|o|u)
        _arguments : "1:bookmark:_tempesta_complete_entries_helper"
        ;;
      move|m)
        _arguments \
          "1:source:_tempesta_complete_entries_helper" \
          "2:destination:_tempesta_complete_entries_helper"
        ;;
      *)
        ;;
    esac
  else
    local -a subcommands
    subcommands=(
      "\[a\]dd:Add a new bookmark"
      "\[e\]dit:Edit an existing bookmark"
      "\[l\]ist:List bookmarks"
      "\[m\]ove:Move an existing bookmark"
      "\[o\]pen:Open a bookmark"
      "\[r\]emove:Remove a bookmark"
      "\[u\]pdate:Update an existing bookmark"
      "" # without this is printing \[
    )
    _describe -t commands 'tempesta' subcommands
  fi
}

# Function to get the bookmark directory from the `tempesta config` command
_get_bookmark_directory() {
  tempesta config | awk -F': ' '/Bookmark store directory:/ {print $2}' | xargs
}

_tempesta_complete_entries_helper () {
  local IFS=$'\n'

  # local prefix="${BOOKMARK_STORE_DIR:-$HOME/.bookmark-store}"
  # Set the prefix dynamically
  local prefix="$(_get_bookmark_directory)"
  local prefix=$(eval echo $prefix)

  _values -C 'bookmarks' ${$(find -L "$prefix" \( -name .git -o -name .gpg-id \) -prune -o -type f -name "*.toml" -print 2>/dev/null | sed -e "s#${prefix}/\{0,1\}##" -e 's#\.toml$##' -e 's#\\#\\\\#g' -e 's#:#\\:#g' | sort):-""}
}

compdef _tempesta tempesta

