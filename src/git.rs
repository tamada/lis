use git2::{Repository, StatusOptions};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_git_statuses(path: &Path) -> HashMap<PathBuf, String> {
    let mut statuses = HashMap::new();
    if let Ok(repo) = Repository::discover(path) {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        if let Ok(repo_statuses) = repo.statuses(Some(&mut opts)) {
            if let Some(workdir) = repo.workdir() {
                let workdir = fs::canonicalize(workdir).unwrap_or_else(|_| workdir.to_path_buf());
                for entry in repo_statuses.iter() {
                    if let Some(path_str) = entry.path() {
                        let full_path = workdir.join(path_str);
                        let status = entry.status();
                        let status_str = if status.is_index_new() || status.is_wt_new() {
                            "A"
                        } else if status.is_index_modified() || status.is_wt_modified() {
                            "M"
                        } else if status.is_index_deleted() || status.is_wt_deleted() {
                            "D"
                        } else if status.is_index_renamed() || status.is_wt_renamed() {
                            "R"
                        } else if status.is_index_typechange() || status.is_wt_typechange() {
                            "T"
                        } else {
                            " "
                        };
                        statuses.insert(full_path, status_str.to_string());
                    }
                }
            }
        }
    }
    statuses
}
