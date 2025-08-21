//! Defines the command-line interface for the fstree application.

use crate::sort;
use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;
use std::path::PathBuf;

/// A blazingly fast, minimalist directory tree viewer, written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(override_usage = "fstree [OPTIONS] [PATH]\n    fstree interactive [OPTIONS] [PATH]")]
pub struct Args {
    /// The subcommand to run. If no subcommand is specified, the classic tree view is displayed.
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// The arguments for the classic tree view. These are used when no subcommand is provided.
    #[command(flatten)]
    pub view: ViewArgs,
}

/// Defines the available subcommands for the application.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start the interactive TUI explorer.
    #[command(visible_alias = "i")]
    Interactive(InteractiveArgs),
}

/// Arguments for the classic `view` command.
#[derive(Parser, Debug, Default)]
pub struct ViewArgs {
    /// The path to the directory to display. Defaults to the current directory.
    #[arg(default_value = ".")]
    pub path: PathBuf,
    /// Specify when to use colorized output.
    #[arg(long, value_name = "WHEN", default_value_t = ColorChoice::Auto)]
    pub color: ColorChoice,
    /// Maximum depth to descend in the directory tree.
    #[arg(short = 'L', long)]
    pub level: Option<usize>,
    /// Display directories only.
    #[arg(short = 'd', long)]
    pub dirs_only: bool,
    /// Display the size of files.
    #[arg(short = 's', long)]
    pub size: bool,
    /// Display file permissions.
    #[arg(short = 'p', long)]
    pub permissions: bool,
    /// Show all files, including hidden ones.
    #[arg(short = 'a', long, help = "Show all files, including hidden ones")]
    pub all: bool,
    /// Respect .gitignore and other standard ignore files.
    #[arg(short = 'g', long)]
    pub gitignore: bool,
    /// Show git status for files and directories.
    #[arg(short = 'G', long)]
    pub git_status: bool,
    /// Display file-specific icons (requires a Nerd Font).
    #[arg(long, help = "Display file-specific icons (requires a Nerd Font)")]
    pub icons: bool,
    /// Render file paths as clickable hyperlinks.
    #[arg(long)]
    pub hyperlinks: bool,
    /// Sort entries by the specified criteria.
    #[arg(long, default_value_t = SortType::Name)]
    pub sort: SortType,
    /// Sort directories before files.
    #[arg(long)]
    pub dirs_first: bool,
    /// Use case-sensitive sorting.
    #[arg(long)]
    pub case_sensitive: bool,
    /// Use natural/version sorting (e.g., file1 < file10).
    #[arg(long)]
    pub natural_sort: bool,
    /// Reverse the sort order.
    #[arg(short = 'r', long)]
    pub reverse: bool,
    /// Sort dotfiles and dotfolders first.
    #[arg(long)]
    pub dotfiles_first: bool,
}

/// Arguments for the `interactive` command.
#[derive(Parser, Debug)]
pub struct InteractiveArgs {
    /// The path to the directory to explore. Defaults to the current directory.
    #[arg(default_value = ".")]
    pub path: PathBuf,
    /// Show all files, including hidden ones.
    #[arg(short = 'a', long)]
    pub all: bool,
    /// Respect .gitignore and other standard ignore files.
    #[arg(short = 'g', long)]
    pub gitignore: bool,
    /// Show git status for files and directories.
    #[arg(short = 'G', long)]
    pub git_status: bool,
    /// Display file-specific icons (requires a Nerd Font).
    #[arg(long)]
    pub icons: bool,
    /// Display the size of files.
    #[arg(short = 's', long)]
    pub size: bool,
    /// Display file permissions.
    #[arg(short = 'p', long)]
    pub permissions: bool,
    /// Initial depth to expand the directory tree.
    #[arg(long, value_name = "LEVEL")]
    pub expand_level: Option<usize>,
    /// Sort entries by the specified criteria.
    #[arg(long, default_value_t = SortType::Name)]
    pub sort: SortType,
    /// Sort directories before files.
    #[arg(long)]
    pub dirs_first: bool,
    /// Use case-sensitive sorting.
    #[arg(long)]
    pub case_sensitive: bool,
    /// Use natural/version sorting (e.g., file1 < file10).
    #[arg(long)]
    pub natural_sort: bool,
    /// Reverse the sort order.
    #[arg(short = 'r', long)]
    pub reverse: bool,
    /// Sort dotfiles and dotfolders first.
    #[arg(long)]
    pub dotfiles_first: bool,
}

/// Defines the available sorting strategies.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum SortType {
    /// Sort by name (default)
    #[default]
    Name,
    /// Sort by file size
    Size,
    /// Sort by modification time
    Modified,
    /// Sort by file extension
    Extension,
}

/// Defines the choices for the --color option.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ColorChoice {
    Always,
    #[default]
    Auto,
    Never,
}

impl From<SortType> for sort::SortType {
    fn from(sort_type: SortType) -> Self {
        match sort_type {
            SortType::Name => sort::SortType::Name,
            SortType::Size => sort::SortType::Size,
            SortType::Modified => sort::SortType::Modified,
            SortType::Extension => sort::SortType::Extension,
        }
    }
}

impl ViewArgs {
    /// Creates a SortOptions instance from the ViewArgs.
    pub fn to_sort_options(&self) -> sort::SortOptions {
        sort::SortOptions {
            sort_type: self.sort.into(),
            directories_first: self.dirs_first,
            case_sensitive: self.case_sensitive,
            natural_sort: self.natural_sort,
            reverse: self.reverse,
            dotfiles_first: self.dotfiles_first,
        }
    }
}

impl InteractiveArgs {
    /// Creates a SortOptions instance from the InteractiveArgs.
    pub fn to_sort_options(&self) -> sort::SortOptions {
        sort::SortOptions {
            sort_type: self.sort.into(),
            directories_first: self.dirs_first,
            case_sensitive: self.case_sensitive,
            natural_sort: self.natural_sort,
            reverse: self.reverse,
            dotfiles_first: self.dotfiles_first,
        }
    }
}

/// Implements the Display trait for SortType to show possible values in help messages.
impl fmt::Display for SortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

/// Implements the Display trait for ColorChoice to show possible values in help messages.
impl fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}
