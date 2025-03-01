_tempesta_completion() {
    local cur subcmd

    # When only the command is entered, complete the subcommands.
    if [ $COMP_CWORD -eq 1 ]; then
        local subcommands="add edit open remove update a e o r u"
        COMPREPLY=( $( compgen -W "$subcommands" -- "${COMP_WORDS[1]}" ) )
        return 0
    fi

    subcmd="${COMP_WORDS[1]}"

    # Handle second argument for all commands except "add" and "a"
    if [[ $COMP_CWORD -eq 2 && $subcmd != 'add' && $subcmd != 'a' ]]; then
        cur="${COMP_WORDS[2]}"
        COMPREPLY=( $( compgen -W "$(_tempesta_complete_entries_helper)" -- "$cur" ) )
        return 0
    fi

    # Handle third argument when the subcommand is "add" or "a"
    if [[ $COMP_CWORD -eq 3 && ( $subcmd == 'add' || $subcmd == 'a' ) ]]; then
        cur="${COMP_WORDS[3]}"
        COMPREPLY=( $( compgen -W "$(_tempesta_complete_entries_helper)" -- "$cur" ) )
        return 0
    fi
}

_tempesta_complete_entries_helper() {
    local prefix="${BOOKMARK_STORE_DIR:-$HOME/.bookmark-store}"
    # Find all *.toml files (ignoring .git and .gpg-id), remove the store prefix and the .toml extension,
    # escape backslashes and colons, and sort the list.
    find -L "$prefix" \( -name .git -o -name .gpg-id \) -prune -o -type f -name "*.toml" -print 2>/dev/null \
      | sed -e "s#${prefix}/\{0,1\}##" -e 's#\.toml$##' -e 's#\\#\\\\#g' -e 's#:#\\:#g' \
      | sort
}

complete -F _tempesta_completion tempesta

