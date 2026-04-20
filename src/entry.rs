use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use chrono::{DateTime, Local};
use uzers::{get_group_by_gid, get_user_by_uid};
use nix::sys::stat::Mode;

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

impl Entry {
    pub fn new(path: &Path, name: String, git_status: String) -> Option<Self> {
        let metadata = fs::symlink_metadata(path).ok()?;
        let is_dir = metadata.is_dir();
        let is_symlink = metadata.file_type().is_symlink();
        let mode = get_mode_string(metadata.permissions().mode(), is_dir, is_symlink);
        let nlink = metadata.nlink();
        let owner = get_user_by_uid(metadata.uid())
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.uid().to_string());
        let group = get_group_by_gid(metadata.gid())
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.gid().to_string());
        let size = metadata.len();
        let modified: DateTime<Local> = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now()).into();
        let modified_str = modified.format("%Y-%m-%d %H:%M").to_string();
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();

        Some(Entry {
            name,
            path: path.to_path_buf(),
            is_dir,
            mode,
            nlink,
            owner,
            group,
            size,
            modified: modified_str,
            git_status,
            extension,
        })
    }
}

fn get_mode_string(mode: u32, is_dir: bool, is_symlink: bool) -> String {
    let mut s = String::with_capacity(10);

    if is_dir {
        s.push('d');
    } else if is_symlink {
        s.push('l');
    } else {
        s.push('-');
    }

    #[cfg(unix)]
    {
        let m = Mode::from_bits_truncate(mode as nix::sys::stat::mode_t);
        s.push(if m.contains(Mode::S_IRUSR) { 'r' } else { '-' });
        s.push(if m.contains(Mode::S_IWUSR) { 'w' } else { '-' });
        s.push(if m.contains(Mode::S_IXUSR) { 'x' } else { '-' });
        s.push(if m.contains(Mode::S_IRGRP) { 'r' } else { '-' });
        s.push(if m.contains(Mode::S_IWGRP) { 'w' } else { '-' });
        s.push(if m.contains(Mode::S_IXGRP) { 'x' } else { '-' });
        s.push(if m.contains(Mode::S_IROTH) { 'r' } else { '-' });
        s.push(if m.contains(Mode::S_IWOTH) { 'w' } else { '-' });
        s.push(if m.contains(Mode::S_IXOTH) { 'x' } else { '-' });
    }

    s
}
