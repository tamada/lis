use git2::{Repository, StatusOptions};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Retrieves the Git status for each file in the repository containing the given path.
///
/// Returns a map from absolute file paths to their status strings.
pub fn get_git_statuses(path: &Path) -> HashMap<PathBuf, String> {
    let mut statuses = HashMap::new();
    if let Ok(repo) = Repository::discover(path) {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        if let (Ok(repo_statuses), Some(workdir)) = (repo.statuses(Some(&mut opts)), repo.workdir())
        {
            let workdir = fs::canonicalize(workdir).unwrap_or_else(|_| workdir.to_path_buf());
            for entry in repo_statuses.iter() {
                if let Some(path_str) = entry.path() {
                    let full_path = workdir.join(path_str);
                    let status_str = get_status_char(entry.status());
                    statuses.insert(full_path, status_str.to_string());
                }
            }
        }
    }
    statuses
}

/// Converts a Git status to a single character string.
fn get_status_char(status: git2::Status) -> &'static str {
    if status.is_index_new() || status.is_wt_new() {
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
    }
}
