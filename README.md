# fstree

A blazing-fast directory tree viewer for terminals—minimalist, interactive, and packed with smart features!

## ✨ Features

- **Speed**: Built for performance, rendering large directory trees in milliseconds.
- **Interactivity**: Navigate your filesystem with intuitive keyboard shortcuts.
- **Customization**: Tailor the appearance and behavior to suit your workflow.

## 🚀 Installation

```bash
cargo install fstree
```

## Usage

```bash
fstree [OPTIONS] [PATH]
```

Note that `PATH` defaults to the current directory (`.`) if not specified.

| Option                 | Description                                                                 |
| :--------------------- | :-------------------------------------------------------------------------- |
| `-a`, `--all`          | List all files and directories, including hidden ones.                      |
| `--color <WHEN>`       | Specify when to use color output (`always`, `auto`, `never`).               |
| `-d`, `--dirs-only`    | List directories only, ignoring all files.                                  |
| `-g`, `--gitignore`    | Respect `.gitignore` and other standard ignore files.                       |
| `-G`, `--git-status`   | Show git status for files and directories.                                  |
| `--icons`              | Display file-specific icons; requires a [Nerd Font](https://www.nerdfonts.com/). |
| `--hyperlinks`         | Render file paths as clickable hyperlinks (classic mode only)               |
| `-L`, `--level <LEVEL>`| Maximum depth to descend.                                                   |
| `-p`, `--permissions`  | Display file permissions (Unix-like systems only).                          |
| `-s`, `--size`         | Display the size of files.                                                  |
| `--sort <TYPE>`        | Sort entries by the specified criteria (`name`, `size`, `modified`, `extension`). |
| `--dirs-first`         | Sort directories before files.                                              |
| `--case-sensitive`     | Use case-sensitive sorting.                                                 |
| `--natural-sort`       | Use natural/version sorting (e.g., file1 < file10).                        |
| `-r`, `--reverse`      | Reverse the sort order.                                                     |
| `--dotfiles-first`     | Sort dotfiles and dotfolders first (dotfolders → folders → dotfiles → files). |
| `--expand-level <LEVEL>`| **Interactive mode only:** Initial depth to expand the interactive tree.   |

-----

## Interactive mode

Launch the TUI with `fstree interactive [OPTIONS] [PATH]`.

### Keyboard controls

| Key(s)  | Action                                                                                                                                      |
| :------ | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `k` | Move selection up. |
| `↓` / `j` | Move selection down. |
| `Enter` | **Context-aware action:**\<br\>- If on a file: Open it in the default editor (`$EDITOR`).\<br\>- If on a directory: Toggle expand/collapse. |
| `q` / `Esc` | Quit the application normally. |
| `Ctrl`+`s` | **Shell integration:** Quits and prints the selected path to stdout. |

## Examples

**1. List the contents of the current directory**

```bash
fstree
```

**2. Explore a project interactively, ignoring gitignored files**

```bash
fstree interactive -g --icons
```

**3. Display a directory with file sizes and permissions (classic view)**

```bash
fstree -sp
```

**4. See the git status of all files in a project**

```bash
fstree -aG
```

**5. Get a tree with clickable file links (in a supported terminal)**

```bash
fstree --hyperlinks
```

**6. Start an interactive session with all data displayed**

```bash
fstree interactive -gG --icons -s -p
```

**7. Sort files naturally with directories first**

```bash
fstree --dirs-first --natural-sort
```

**8. Sort by file size in descending order**

```bash
fstree --sort size --reverse
```

**9. Sort by extension with case-sensitive ordering**

```bash
fstree --sort extension --case-sensitive
```

**10. Sort with dotfiles first and directories first**

```bash
fstree --dotfiles-first --dirs-first -a
```

## Piping and shell interaction

The classic `view` mode is designed to work well with other command-line tools via pipes (`|`).

### Interactive fuzzy finding with `fzf`

This is a powerful way to instantly find any file in a large project.

```bash
fstree -a -g --icons | fzf
```

`fzf` will take the tree from `fstree` and provide an interactive search prompt to filter it.

### Paging large trees with `less` or `bat`

If a directory is too large to fit on one screen, pipe the output to a *pager*.

```bash
# Using less (the -R flag preserves color)
fstree -L 10 | less -R

# Using bat (a modern pager that understands colors)
fstree --icons | bat
```

### Changing directories with `fstree`

You can use `fstree` as a visual `cd` command. Add the following function to your shell's startup file (e.g., `~/.bashrc`, `~/.zshrc`):

```bash
# A function to visually change directories with fstree
lcd() {
    # Run fstree and capture the selected path into a variable.
    # The TUI will draw on stderr, and the final path will be on stdout.
    local selected_dir
    selected_dir="$(fstree interactive -g --icons)"

    # If the user selected a path (and didn't just quit), `cd` into it.
    # Check if the selection is a directory.
    if [[ -n "$selected_dir" && -d "$selected_dir" ]]; then
        cd "$selected_dir"
    fi
}
```

After adding this and starting a new shell session (or running `source ~/.bashrc`), you can simply run:

```bash
lcd
```

This will launch the `fstree` interactive UI. Navigate to the directory you want, press `Ctrl+s`, and your shell's current directory will instantly change.

## Color customization

`fstree` respects your terminal's color theme by default. It reads the `LS_COLORS` environment variable to colorize files and directories according to your system's configuration. This is the same variable used by GNU `ls` and other modern command-line tools.

### Linux

On most Linux distributions, this variable is already set. You can customize it by modifying your shell's startup file.

### macOS

macOS does not set the `LS_COLORS` variable by default. To enable this feature, you can install `coreutils`:

```bash
brew install coreutils
````

Then, add the following line to your shell's startup file (e.g., `~/.zshrc` or `~/.bash_profile`):

```bash
# Use gdircolors from the newly installed coreutils
eval "$(gdircolors)"
```

### Windows

Windows does not use the `LS_COLORS` variable natively, but you can set it manually to enable color support in modern terminals like Windows Terminal.

First, copy a standard `LS_COLORS` string, such as this one:
`rs=0:di=01;34:ln=01;36:ex=01;32:*.zip=01;31:*.png=01;35:`. This string defines colors for various file types:

* **Directories:** Displayed in **bold blue**.
* **Executable files:** Displayed in **bold green** (e.g., `.sh` scripts).
* **Symbolic links:** Displayed in **bold cyan**.
* **Archives:** Displayed in **bold red** (e.g., `.zip`, `.tar.gz`).
* **Image files:** Displayed in **bold magenta** (e.g., `.png`, `.jpg`).
* **Other files:** Displayed in the terminal's default text color.

To set it for your current **PowerShell** session, run:

```powershell
$env:LS_COLORS="rs=0:di=01;34:ln=01;36:ex=01;32:*.zip=01;31:*.png=01;35:"
```

To set it for your current **Command Prompt** (cmd) session, run:

```cmd
set LS_COLORS=rs=0:di=01;34:ln=01;36:ex=01;32:*.zip=01;31:*.png=01;35:
```

To make the setting permanent, you can add the command to your PowerShell profile or set it in the system's "Environment Variables" dialog.

After setting the variable and starting a new shell session, `fstree` will automatically display your configured colors.

## Inspiration

The philosophy and functionality of `fstree` are heavily inspired by the excellent C-based [tree](https://github.com/Old-Man-Programmer/tree) command line program. This project is an attempt to recreate that classic utility in modern, safe Rust.

## 🙏 Credits

- [tree](https://github.com/Old-Man-Programmer/tree)

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.