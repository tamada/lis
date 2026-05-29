//! `lis` is a library and CLI tool for listing directory entries.
//!
//! It provides features like Git status, icons, and multiple output formats.

pub mod entry;
pub mod git;

use entry::Entry;
use git::get_git_statuses;
use ignore::{Walk, WalkBuilder};
use std::fs;
use std::path::{Path, PathBuf};

/// The main entry point for listing directory contents.
///
/// `Lis` uses a builder pattern to configure how directory entries should be retrieved.
///
/// # Examples
///
/// ```
/// use lis::Lis;
/// use std::path::PathBuf;
///
/// let lis = Lis::new(PathBuf::from("."))
///     .all(true)
///     .recursive(false);
///
/// let entries = lis.list();
/// ```
pub struct Lis {
    path: PathBuf,
    recursive: bool,
    all: bool,
    whole_all: bool,
}

impl Lis {
    /// Creates a new `Lis` instance for the given path.
    ///
    /// # Examples
    ///
    /// ```
    /// use lis::Lis;
    /// use std::path::PathBuf;
    ///
    /// let lis = Lis::new(PathBuf::from("."));
    /// ```
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            recursive: false,
            all: false,
            whole_all: false,
        }
    }

    /// Sets whether to list entries recursively.
    ///
    /// # Examples
    ///
    /// ```
    /// use lis::Lis;
    /// use std::path::PathBuf;
    ///
    /// let lis = Lis::new(PathBuf::from(".")).recursive(true);
    /// ```
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Sets whether to list hidden entries while respecting `.gitignore`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lis::Lis;
    /// use std::path::PathBuf;
    ///
    /// let lis = Lis::new(PathBuf::from(".")).all(true);
    /// ```
    pub fn all(mut self, all: bool) -> Self {
        self.all = all;
        self
    }

    /// Sets whether to list all entries, ignoring `.gitignore`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lis::Lis;
    /// use std::path::PathBuf;
    ///
    /// let lis = Lis::new(PathBuf::from(".")).whole_all(true);
    /// ```
    pub fn whole_all(mut self, whole_all: bool) -> Self {
        self.whole_all = whole_all;
        self
    }

    /// Lists the entries in the directory according to the configured options.
    ///
    /// Returns a vector of [`Entry`](crate::entry::Entry) objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use lis::Lis;
    /// use std::path::PathBuf;
    ///
    /// let lis = Lis::new(PathBuf::from("."));
    /// let entries = lis.list();
    /// ```
    pub fn list(&self) -> Vec<Entry> {
        let git_statuses = get_git_statuses(&self.path);
        let mut entries = Vec::new();
        let walker = self.create_walker();

        for dir_entry in walker.flatten() {
            if let Some(entry) = self.process_dir_entry(&dir_entry, &git_statuses) {
                entries.push(entry);
            }
        }
        entries
    }

    /// Creates a walker based on the current configuration.
    fn create_walker(&self) -> Walk {
        let mut builder = WalkBuilder::new(&self.path);
        builder.hidden(!self.all && !self.whole_all);
        builder.git_ignore(!self.whole_all);
        builder.max_depth(if self.recursive { None } else { Some(1) });
        builder.build()
    }

    /// Processes a single directory entry and converts it to an `Entry` struct.
    fn process_dir_entry(
        &self,
        dir_entry: &ignore::DirEntry,
        git_statuses: &std::collections::HashMap<PathBuf, String>,
    ) -> Option<Entry> {
        let path = dir_entry.path();
        let name = self.get_display_name(path, dir_entry)?;

        let abs_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let git_status = git_statuses
            .get(&abs_path)
            .cloned()
            .unwrap_or_else(|| " ".to_string());

        Entry::new(path, name, git_status)
    }

    /// Determines the display name for a directory entry based on whether listing is recursive.
    fn get_display_name(&self, path: &Path, dir_entry: &ignore::DirEntry) -> Option<String> {
        if self.recursive {
            match path.strip_prefix(&self.path) {
                Ok(p) => {
                    let s = p.to_string_lossy().into_owned();
                    Some(if s.is_empty() {
                        self.path.to_string_lossy().into_owned()
                    } else {
                        s
                    })
                }
                Err(_) => Some(dir_entry.file_name().to_string_lossy().into_owned()),
            }
        } else {
            if path == self.path {
                return None;
            }
            Some(dir_entry.file_name().to_string_lossy().into_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lis_new() {
        let path = PathBuf::from(".");
        let lis = Lis::new(path.clone());
        assert_eq!(lis.path, path);
        assert!(!lis.recursive);
        assert!(!lis.all);
        assert!(!lis.whole_all);
    }

    #[test]
    fn test_lis_builder() {
        let lis = Lis::new(PathBuf::from("."))
            .recursive(true)
            .all(true)
            .whole_all(true);
        assert!(lis.recursive);
        assert!(lis.all);
        assert!(lis.whole_all);
    }
}
