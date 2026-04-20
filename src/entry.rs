use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use chrono::{DateTime, Local};
use uzers::{get_group_by_gid, get_user_by_uid};
use nix::sys::stat::Mode;
use clap::ValueEnum;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub mode: String,
    pub nlink: u64,
    pub owner: String,
    pub group: String,
    pub size: u64,
    pub modified: String,
    pub git_status: String,
    pub extension: String,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    Name,
    Time,
    Size,
    Extension,
}

impl Entry {
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
            extension: path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string(),
        })
    }
}

pub fn sort_entries(entries: &mut [Entry], sort_by: SortBy, reverse: bool) {
    entries.sort_by(|a, b| {
        let cmp = match sort_by {
            SortBy::Name => a.name.cmp(&b.name),
            SortBy::Time => a.modified.cmp(&b.modified),
            SortBy::Size => a.size.cmp(&b.size),
            SortBy::Extension => a.extension.cmp(&b.extension),
        };
        if reverse {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

fn get_owner_name(uid: u32) -> String {
    get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string())
}

fn get_group_name(gid: u32) -> String {
    get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| gid.to_string())
}

fn get_modified_time(metadata: &fs::Metadata) -> String {
    let modified: DateTime<Local> = metadata.modified()
        .unwrap_or_else(|_| std::time::SystemTime::now())
        .into();
    modified.format("%Y-%m-%d %H:%M").to_string()
}

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

fn get_file_type_char(is_dir: bool, is_symlink: bool) -> char {
    if is_dir { 'd' } else if is_symlink { 'l' } else { '-' }
}

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
