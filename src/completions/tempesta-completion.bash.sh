_tempesta() {
    local cur subcmd

    # When only the command is entered, complete the subcommands.
    if [ $COMP_CWORD -eq 1 ]; then
        local subcommands="add edit list move native-host open remove update a e l m o r u"
        COMPREPLY=( $( compgen -W "$subcommands" -- "${COMP_WORDS[1]}" ) )
        return 0
    fi

    subcmd="${COMP_WORDS[1]}"

    if [[ $COMP_CWORD -eq 2 && $subcmd == 'native-host' ]]; then
        COMPREPLY=( $( compgen -W "install" -- "${COMP_WORDS[2]}" ) )
        return 0
    fi

    # Handle second argument for all commands
    if [[ $COMP_CWORD -eq 2 ]]; then
        _tempesta_complete_entries "${COMP_WORDS[2]}"
        return 0
    fi

    if [[ $COMP_CWORD -eq 4 && ( $subcmd == 'move' || $subcmd == 'm' ) ]]; then
        _tempesta_complete_entries "${COMP_WORDS[4]}"
        return 0
    fi
}

# Complete one path level at a time: subdirectories (with a trailing slash)
# and bookmarks (without the .toml extension) directly under the typed path.
_tempesta_complete_entries() {
    local cur="$1"
    local prefix="$(_get_bookmark_directory)"
    prefix=$(eval echo $prefix)
    prefix="${prefix%/}/"

    local IFS=$'\n'
    local items=( $(compgen -f -- "$prefix$cur") )
    local item has_dir=0
    COMPREPLY=()
    for item in "${items[@]}"; do
        # Skip hidden entries such as .git and .gpg-id
        [[ $item =~ /\.[^/]*$ ]] && continue
        if [[ -d $item ]]; then
            COMPREPLY+=( "${item#$prefix}/" )
            has_dir=1
        elif [[ $item == *.toml ]]; then
            item="${item%.toml}"
            COMPREPLY+=( "${item#$prefix}" )
        fi
    done

    # Add a trailing space only for a single bookmark match, so that a
    # directory can be completed further
    if [[ ${#COMPREPLY[@]} -gt 1 || $has_dir -eq 1 ]]; then
        compopt -o nospace
    fi
}

# Function to get the bookmark directory from the `tempesta config` command
_get_bookmark_directory() {
  tempesta config | awk -F': ' '/Bookmark store directory:/ {print $2}' | xargs
}

# Register for the binary and its aliases
complete -F _tempesta tempesta t tmps

