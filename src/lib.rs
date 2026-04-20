pub mod entry;
pub mod git;

use entry::Entry;
use git::get_git_statuses;
use ignore::{Walk, WalkBuilder};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Lis {
    path: PathBuf,
    recursive: bool,
    all: bool,
    whole_all: bool,
}

impl Lis {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            recursive: false,
            all: false,
            whole_all: false,
        }
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub fn all(mut self, all: bool) -> Self {
        self.all = all;
        self
    }

    pub fn whole_all(mut self, whole_all: bool) -> Self {
        self.whole_all = whole_all;
        self
    }

    pub fn list(&self) -> Vec<Entry> {
        let git_statuses = get_git_statuses(&self.path);
        let mut entries = Vec::new();
        let walker = self.create_walker();

        for result in walker {
            if let Ok(dir_entry) = result {
                if let Some(entry) = self.process_dir_entry(&dir_entry, &git_statuses) {
                    entries.push(entry);
                }
            }
        }
        entries
    }

    fn create_walker(&self) -> Walk {
        let mut builder = WalkBuilder::new(&self.path);
        builder.hidden(!self.all && !self.whole_all);
        builder.git_ignore(!self.whole_all);
        builder.max_depth(if self.recursive { None } else { Some(1) });
        builder.build()
    }

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
