function _tempesta_complete_entries_helper
    set -l prefix (string replace -r '^$' $HOME/.bookmark-store -- "$BOOKMARK_STORE_DIR")
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
complete -c tempesta -n '__fish_use_subcommand' -a "open" -d "Open a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "move" -d "Move a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "remove" -d "Remove a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "update" -d "Update a bookmark"
complete -c tempesta -n '__fish_use_subcommand' -a "a" -d "Shortcut for add"
complete -c tempesta -n '__fish_use_subcommand' -a "e" -d "Shortcut for edit"
complete -c tempesta -n '__fish_use_subcommand' -a "m" -d "Shortcut for move"
complete -c tempesta -n '__fish_use_subcommand' -a "o" -d "Shortcut for open"
complete -c tempesta -n '__fish_use_subcommand' -a "r" -d "Shortcut for remove"
complete -c tempesta -n '__fish_use_subcommand' -a "u" -d "Shortcut for update"

# Completion for entries (for all subcommands except "add" and "a")
for cmd in edit open remove update e o r u
    complete -c tempesta -n "__fish_seen_subcommand_from $cmd; and not __fish_seen_subcommand_from add a" -a "(_tempesta_complete_entries)"
end

# Completion for entries as the 3rd argument for "add" and "a"
for cmd in add a
    complete -c tempesta -n "__fish_seen_subcommand_from $cmd; and count (commandline -opc) = 3" -a "(_tempesta_complete_entries)"
end

