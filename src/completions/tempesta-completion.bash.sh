_tempesta() {
    local cur subcmd

    # When only the command is entered, complete the subcommands.
    if [ $COMP_CWORD -eq 1 ]; then
        local subcommands="add edit move open remove update a e m o r u"
        COMPREPLY=( $( compgen -W "$subcommands" -- "${COMP_WORDS[1]}" ) )
        return 0
    fi

    subcmd="${COMP_WORDS[1]}"

    # Handle second argument for all commands
    if [[ $COMP_CWORD -eq 2 ]]; then
        cur="${COMP_WORDS[2]}"
        COMPREPLY=( $( compgen -W "$(_tempesta_complete_entries_helper)" -- "$cur" ) )
        return 0
    fi

    if [[ $COMP_CWORD -eq 4 && ( $subcmd == 'move' || $subcmd == 'm' ) ]]; then
        cur="${COMP_WORDS[4]}"
        COMPREPLY=( $( compgen -W "$(_tempesta_complete_entries_helper)" -- "$cur" ) )
        return 0
    fi
}

# Function to get the bookmark directory from the `tempesta config` command
_get_bookmark_directory() {
  tempesta config | awk -F': ' '/Bookmark store directory:/ {print $2}' | xargs
}

_tempesta_complete_entries_helper() {
    # local prefix="${BOOKMARK_STORE_DIR:-$HOME/.bookmark-store}"
    # Set the prefix dynamically
    local prefix="$(_get_bookmark_directory)"

    # Find all *.toml files (ignoring .git and .gpg-id), remove the store prefix and the .toml extension,
    # escape backslashes and colons, and sort the list.
    find -L "$prefix" \( -name .git -o -name .gpg-id \) -prune -o -type f -name "*.toml" -print 2>/dev/null \
      | sed -e "s#${prefix}/\{0,1\}##" -e 's#\.toml$##' -e 's#\\#\\\\#g' -e 's#:#\\:#g' \
      | sort
}

complete -F _tempesta tempesta

