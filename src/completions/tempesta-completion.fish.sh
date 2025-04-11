function _get_bookmark_directory
    tempesta config | string match -r 'Bookmark store directory: *(.+)' | string replace -r 'Bookmark store directory: *' ''
end

function _tempesta_complete_entries_helper
    # Dynamically infer the prefix from the tempesta config command
    set -l prefix (_get_bookmark_directory)
    # Expand tilde if present
    if string match -q "~*" "$prefix"
      set prefix (eval echo $prefix)
    end

    find -L "$prefix" \( -name .git -o -name .gpg-id \) -prune -o -type f -name "*.toml" -print 2>/dev/null \
    | sed -e "s#${prefix}/\{0,1\}##" -e 's#\.toml$##' -e 's#\\#\\\\#g' -e 's#:#\\:#g' \
    | sort
end

function _tempesta_complete_entries
    _tempesta_complete_entries_helper
end

# Subcommands
complete -c tempesta -n '__fish_use_subcommand' -a "add" -d "Add a new bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "edit" -d "Edit a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "list" -d "List bookmarks"
complete -c tempesta -n '__fish_use_subcommand' -a "move" -d "Move a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "open" -d "Open a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "remove" -d "Remove a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "update" -d "Update a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "a" -d "Shortcut for add"
complete -c tempesta -n '__fish_use_subcommand' -a "e" -d "Shortcut for edit"
complete -c tempesta -n '__fish_use_subcommand' -a "l" -d "Shortcut for list"
complete -c tempesta -n '__fish_use_subcommand' -a "m" -d "Shortcut for move"
complete -c tempesta -n '__fish_use_subcommand' -a "o" -d "Shortcut for open"
complete -c tempesta -n '__fish_use_subcommand' -a "r" -d "Shortcut for remove"
complete -c tempesta -n '__fish_use_subcommand' -a "u" -d "Shortcut for update"

# Completion for entries for all subcommands
for cmd in add edit list move open remove update a e l m o r u
    complete -c tempesta -n "__fish_seen_subcommand_from $cmd" -a "(_tempesta_complete_entries)"
end

