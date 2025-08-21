//! Provides functionality for interacting with Git repositories.
//!
//! This module uses the `git2` crate to discover repositories, read file statuses,
//! and provide a simplified representation of those statuses for display.

use git2::Repository;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A simplified representation of a file's Git status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileStatus {
    Modified,
    New,
    Deleted,
    Renamed,
    Typechange,
    Untracked,
    Conflicted,
}

impl FileStatus {
    /// Returns the character symbol for the status.
    pub fn get_char(&self) -> char {
        match self {
            Self::Modified => 'M',
            Self::New => 'A', // For "Added"
            Self::Deleted => 'D',
            Self::Renamed => 'R',
            Self::Typechange => 'T',
            Self::Untracked => '?',
            Self::Conflicted => 'C',
        }
    }
}

/// A cache mapping file paths to their Git status.
pub type StatusCache = HashMap<PathBuf, FileStatus>;

/// Contains the status cache and the root path of the repository.
#[derive(Clone)]
pub struct GitRepoStatus {
    pub cache: StatusCache,
    pub root: PathBuf,
}

/// Discovers a Git repository from a starting path, scans for file statuses,
/// and returns them in a `GitRepoStatus` object.
///
/// The cache will contain paths relative to the repository root.
/// If no Git repository is found, it returns `Ok(None)`.
pub fn load_status(start_path: &Path) -> anyhow::Result<Option<GitRepoStatus>> {
    let Ok(repo) = Repository::discover(start_path) else {
        return Ok(None);
    };

    let Some(workdir) = repo.workdir() else {
        return Ok(None);
    };

    let mut cache = StatusCache::new();
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true).include_ignored(false).recurse_untracked_dirs(true);

    let statuses = repo.statuses(Some(&mut opts))?;

    for entry in statuses.iter() {
        let Some(status) = git_to_file_status(entry.status()) else {
            continue;
        };

        if let Some(path_str) = entry.path() {
            // Use the relative path directly as the key.
            cache.insert(PathBuf::from(path_str), status);
        }
    }

    // Return the CANONICALIZED workdir path as the root.
    Ok(Some(GitRepoStatus { cache, root: workdir.canonicalize()? }))
}

/// Converts a `git2::Status` bitflag into our simplified `FileStatus` enum.
fn git_to_file_status(s: git2::Status) -> Option<FileStatus> {
    if s.is_conflicted() {
        return Some(FileStatus::Conflicted);
    }
    if s.is_index_new() {
        return Some(FileStatus::New);
    }
    if s.is_index_modified() {
        return Some(FileStatus::Modified);
    }
    if s.is_index_deleted() {
        return Some(FileStatus::Deleted);
    }
    if s.is_index_renamed() {
        return Some(FileStatus::Renamed);
    }
    if s.is_index_typechange() {
        return Some(FileStatus::Typechange);
    }
    if s.is_wt_new() {
        return Some(FileStatus::Untracked);
    }
    if s.is_wt_modified() {
        return Some(FileStatus::Modified);
    }
    if s.is_wt_deleted() {
        return Some(FileStatus::Deleted);
    }
    if s.is_wt_renamed() {
        return Some(FileStatus::Renamed);
    }
    if s.is_wt_typechange() {
        return Some(FileStatus::Typechange);
    }
    None
}
