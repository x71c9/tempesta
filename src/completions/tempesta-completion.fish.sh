function _get_bookmark_directory
    # -f keeps only the line where the replacement happened
    tempesta config | string replace -rf '^Bookmark store directory: *' ''
end

function _tempesta_complete_entries_helper
    # Dynamically infer the prefix from the tempesta config command
    set -l prefix (_get_bookmark_directory)
    # Expand tilde if present
    if string match -q "~*" "$prefix"
      set prefix (eval echo $prefix)
    end

    # Find all *.toml files (ignoring .git and .gpg-id), remove the store prefix
    # and the .toml extension, escape backslashes and colons, and sort the list.
    # Inside fish single quotes a literal backslash is written as \\.
    find -L "$prefix" \( -name .git -o -name .gpg-id \) -prune -o -type f -name "*.toml" -print 2>/dev/null \
    | sed -e "s#$prefix/\{0,1\}##" -e 's#\.toml$##' -e 's#\\\\#\\\\\\\\#g' -e 's#:#\\\\:#g' \
    | sort
end

function _tempesta_complete_entries
    _tempesta_complete_entries_helper
end

# Register for the binary and its aliases
for bin in tempesta t tmps
    # Subcommands
    complete -c $bin -n '__fish_use_subcommand' -a "add" -d "Add a new bookmark"
    complete -c $bin -n '__fish_use_subcommand' -a "edit" -d "Edit a bookmark"
    complete -c $bin -n '__fish_use_subcommand' -a "list" -d "List bookmarks"
    complete -c $bin -n '__fish_use_subcommand' -a "move" -d "Move a bookmark"
    complete -c $bin -n '__fish_use_subcommand' -a "open" -d "Open a bookmark"
    complete -c $bin -n '__fish_use_subcommand' -a "remove" -d "Remove a bookmark"
    complete -c $bin -n '__fish_use_subcommand' -a "update" -d "Update a bookmark"
    complete -c $bin -n '__fish_use_subcommand' -a "a" -d "Shortcut for add"
    complete -c $bin -n '__fish_use_subcommand' -a "e" -d "Shortcut for edit"
    complete -c $bin -n '__fish_use_subcommand' -a "l" -d "Shortcut for list"
    complete -c $bin -n '__fish_use_subcommand' -a "m" -d "Shortcut for move"
    complete -c $bin -n '__fish_use_subcommand' -a "o" -d "Shortcut for open"
    complete -c $bin -n '__fish_use_subcommand' -a "r" -d "Shortcut for remove"
    complete -c $bin -n '__fish_use_subcommand' -a "u" -d "Shortcut for update"

    # Completion for entries for all subcommands
    for cmd in add edit list move open remove update a e l m o r u
        complete -c $bin -n "__fish_seen_subcommand_from $cmd" -a "(_tempesta_complete_entries)"
    end
end

