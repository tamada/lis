use git2::{Repository, StatusOptions};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Retrieves the Git status for each file in the repository containing the given path.
///
/// Returns a map from absolute file paths to their status strings.
///
/// # Examples
///
/// ```
/// use lis::git::get_git_statuses;
/// use std::path::Path;
///
/// let statuses = get_git_statuses(Path::new("."));
/// ```
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
///
/// Returns "A" for new files, "M" for modified, "D" for deleted, "R" for renamed,
/// "T" for type change, and " " otherwise.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status_char() {
        // Test various git2::Status combinations
        assert_eq!(get_status_char(git2::Status::INDEX_NEW), "A");
        assert_eq!(get_status_char(git2::Status::WT_NEW), "A");
        assert_eq!(get_status_char(git2::Status::INDEX_MODIFIED), "M");
        assert_eq!(get_status_char(git2::Status::WT_MODIFIED), "M");
        assert_eq!(get_status_char(git2::Status::INDEX_DELETED), "D");
        assert_eq!(get_status_char(git2::Status::WT_DELETED), "D");
        assert_eq!(get_status_char(git2::Status::INDEX_RENAMED), "R");
        assert_eq!(get_status_char(git2::Status::WT_RENAMED), "R");
        assert_eq!(get_status_char(git2::Status::INDEX_TYPECHANGE), "T");
        assert_eq!(get_status_char(git2::Status::WT_TYPECHANGE), "T");
        assert_eq!(get_status_char(git2::Status::CURRENT), " ");
    }
}
