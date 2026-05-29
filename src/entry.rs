//! Data structures and sorting logic for directory entries.
//!
//! This module defines the [`Entry`] struct, which encapsulates metadata for a file
//! or directory, and the [`SortBy`] enum for configuring how entries are ordered.

use chrono::{DateTime, Local};
use clap::ValueEnum;
use nix::sys::stat::Mode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uzers::{get_group_by_gid, get_user_by_uid};
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

/// Represents a single directory entry with its metadata.
///
/// # Examples
///
/// ```
/// use lis::entry::Entry;
/// use std::path::Path;
///
/// // Note: In practice, you would pass an existing path.
/// // let entry = Entry::new(Path::new("Cargo.toml"), "Cargo.toml".to_string(), " ".to_string());
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    /// The display name of the entry.
    pub name: String,
    /// The full path to the entry.
    pub path: PathBuf,
    /// Whether the entry is a directory.
    pub is_dir: bool,
    /// The file mode string (e.g., "-rw-r--r--").
    pub mode: String,
    /// The number of hard links to the entry.
    pub nlink: u64,
    /// The name of the owner.
    pub owner: String,
    /// The name of the group.
    pub group: String,
    /// The size of the entry in bytes.
    pub size: u64,
    /// The last modification time string.
    pub modified: String,
    /// The Git status of the entry (e.g., "M", "A", or " ").
    pub git_status: String,
    /// The file extension.
    pub extension: String,
}

/// Available sorting options for directory entries.
///
/// # Examples
///
/// ```
/// use lis::entry::SortBy;
///
/// let sort = SortBy::Name;
/// ```
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    /// Sort by name (ascending).
    Name,
    /// Sort by last modification time (ascending).
    Time,
    /// Sort by size in bytes (ascending).
    Size,
    /// Sort by file extension (ascending).
    Extension,
}

impl Entry {
    /// Creates a new `Entry` instance by reading metadata from the filesystem.
    ///
    /// Returns `None` if the metadata cannot be retrieved.
    ///
    /// # Examples
    ///
    /// ```
    /// use lis::entry::Entry;
    /// use std::path::Path;
    ///
    /// let entry = Entry::new(Path::new("Cargo.toml"), "Cargo.toml".to_string(), " ".to_string());
    /// ```
    pub fn new(path: &Path, name: String, git_status: String) -> Option<Self> {
        let metadata = fs::symlink_metadata(path).ok()?;
        let is_dir = metadata.is_dir();
        let is_symlink = metadata.file_type().is_symlink();

        Some(Entry {
            name,
            path: path.to_path_buf(),
            is_dir,
            mode: get_mode_string(metadata.permissions().mode(), is_dir, is_symlink),
            nlink: metadata.nlink(),
            owner: get_owner_name(metadata.uid()),
            group: get_group_name(metadata.gid()),
            size: metadata.len(),
            modified: get_modified_time(&metadata),
            git_status,
            extension: path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string(),
        })
    }
}

/// Sorts the given slice of entries based on the specified criteria.
///
/// # Examples
///
/// ```
/// use lis::entry::{Entry, SortBy, sort_entries};
/// use std::path::PathBuf;
///
/// let mut entries = Vec::new();
/// // ... populate entries ...
/// sort_entries(&mut entries, SortBy::Name, false);
/// ```
pub fn sort_entries(entries: &mut [Entry], sort_by: SortBy, reverse: bool) {
    entries.sort_by(|a, b| {
        let cmp = match sort_by {
            SortBy::Name => a.name.cmp(&b.name),
            SortBy::Time => a.modified.cmp(&b.modified),
            SortBy::Size => a.size.cmp(&b.size),
            SortBy::Extension => a.extension.cmp(&b.extension),
        };
        if reverse { cmp.reverse() } else { cmp }
    });
}

/// Retrieves the owner name for the given UID.
fn get_owner_name(uid: u32) -> String {
    get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string())
}

/// Retrieves the group name for the given GID.
fn get_group_name(gid: u32) -> String {
    get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| gid.to_string())
}

/// Retrieves the last modification time of the given metadata as a formatted string.
fn get_modified_time(metadata: &fs::Metadata) -> String {
    let modified: DateTime<Local> = metadata
        .modified()
        .unwrap_or_else(|_| std::time::SystemTime::now())
        .into();
    modified.format("%Y-%m-%d %H:%M").to_string()
}

/// Constructs a file mode string for the given metadata.
fn get_mode_string(mode: u32, is_dir: bool, is_symlink: bool) -> String {
    let mut s = String::with_capacity(10);
    s.push(get_file_type_char(is_dir, is_symlink));

    #[cfg(unix)]
    {
        let m = Mode::from_bits_truncate(mode as nix::sys::stat::mode_t);
        s.push_str(&get_permissions_string(m));
    }
    s
}

/// Returns the file type character ('d', 'l', or '-') for the entry.
fn get_file_type_char(is_dir: bool, is_symlink: bool) -> char {
    if is_dir {
        'd'
    } else if is_symlink {
        'l'
    } else {
        '-'
    }
}

/// Constructs the permissions part of the mode string.
#[cfg(unix)]
fn get_permissions_string(m: Mode) -> String {
    let mut s = String::with_capacity(9);
    s.push(if m.contains(Mode::S_IRUSR) { 'r' } else { '-' });
    s.push(if m.contains(Mode::S_IWUSR) { 'w' } else { '-' });
    s.push(if m.contains(Mode::S_IXUSR) { 'x' } else { '-' });
    s.push(if m.contains(Mode::S_IRGRP) { 'r' } else { '-' });
    s.push(if m.contains(Mode::S_IWGRP) { 'w' } else { '-' });
    s.push(if m.contains(Mode::S_IXGRP) { 'x' } else { '-' });
    s.push(if m.contains(Mode::S_IROTH) { 'r' } else { '-' });
    s.push(if m.contains(Mode::S_IWOTH) { 'w' } else { '-' });
    s.push(if m.contains(Mode::S_IXOTH) { 'x' } else { '-' });
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_entry(name: &str, size: u64, modified: &str, extension: &str) -> Entry {
        Entry {
            name: name.to_string(),
            path: PathBuf::from(name),
            is_dir: false,
            mode: "-rw-r--r--".to_string(),
            nlink: 1,
            owner: "user".to_string(),
            group: "group".to_string(),
            size,
            modified: modified.to_string(),
            git_status: " ".to_string(),
            extension: extension.to_string(),
        }
    }

    #[test]
    fn test_sort_entries_by_name() {
        let mut entries = vec![
            create_test_entry("b", 20, "2023-01-01 10:00", "txt"),
            create_test_entry("a", 10, "2023-01-01 11:00", "rs"),
            create_test_entry("c", 30, "2023-01-01 09:00", "md"),
        ];

        sort_entries(&mut entries, SortBy::Name, false);
        assert_eq!(entries[0].name, "a");
        assert_eq!(entries[1].name, "b");
        assert_eq!(entries[2].name, "c");

        sort_entries(&mut entries, SortBy::Name, true);
        assert_eq!(entries[0].name, "c");
        assert_eq!(entries[1].name, "b");
        assert_eq!(entries[2].name, "a");
    }

    #[test]
    fn test_sort_entries_by_size() {
        let mut entries = vec![
            create_test_entry("b", 20, "2023-01-01 10:00", "txt"),
            create_test_entry("a", 10, "2023-01-01 11:00", "rs"),
            create_test_entry("c", 30, "2023-01-01 09:00", "md"),
        ];

        sort_entries(&mut entries, SortBy::Size, false);
        assert_eq!(entries[0].size, 10);
        assert_eq!(entries[1].size, 20);
        assert_eq!(entries[2].size, 30);
    }

    #[test]
    fn test_get_file_type_char() {
        assert_eq!(get_file_type_char(true, false), 'd');
        assert_eq!(get_file_type_char(false, true), 'l');
        assert_eq!(get_file_type_char(false, false), '-');
    }
}
