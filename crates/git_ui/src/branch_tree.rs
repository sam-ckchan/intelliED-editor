//! The rows of the Git log's branches tree, as in IntelliJ: the current
//! branch, then local and remote branches, with `/` in a name as folders.

use std::collections::BTreeMap;

use git::repository::Branch;
use gpui::SharedString;
use rustc_hash::FxHashSet as HashSet;

const LOCAL_GROUP: &str = "Local";
const REMOTE_GROUP: &str = "Remote";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BranchTreeRow {
    /// "HEAD (Current Branch)", with the checked-out branch's ref.
    Head {
        ref_name: SharedString,
    },
    /// "Local", "Remote", a remote, or a folder. `key` is its path from the root.
    Group {
        key: SharedString,
        label: SharedString,
        depth: usize,
        expanded: bool,
    },
    Branch {
        ref_name: SharedString,
        label: SharedString,
        depth: usize,
        is_head: bool,
    },
}

#[derive(Default)]
struct Node {
    branches: Vec<(String, SharedString, bool)>,
    folders: BTreeMap<String, Node>,
}

impl Node {
    fn insert(&mut self, path: &str, ref_name: &SharedString, is_head: bool) {
        match path.split_once('/') {
            Some((folder, rest)) => {
                self.folders.entry(folder.to_string()).or_default().insert(rest, ref_name, is_head);
            }
            None => self.branches.push((path.to_string(), ref_name.clone(), is_head)),
        }
    }

    fn is_empty(&self) -> bool {
        self.branches.is_empty() && self.folders.is_empty()
    }

    /// Branches first (the checked-out one on top), then folders, as IntelliJ lists them.
    fn flatten(
        mut self,
        key: &str,
        depth: usize,
        expand_all: bool,
        collapsed: &HashSet<SharedString>,
        rows: &mut Vec<BranchTreeRow>,
    ) {
        self.branches.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
        for (label, ref_name, is_head) in self.branches {
            rows.push(BranchTreeRow::Branch { ref_name, label: label.into(), depth, is_head });
        }
        for (label, folder) in self.folders {
            folder.flatten_group(&format!("{key}/{label}"), label, depth, expand_all, collapsed, rows);
        }
    }

    fn flatten_group(
        self,
        key: &str,
        label: String,
        depth: usize,
        expand_all: bool,
        collapsed: &HashSet<SharedString>,
        rows: &mut Vec<BranchTreeRow>,
    ) {
        let key_string = SharedString::from(key.to_string());
        let expanded = expand_all || !collapsed.contains(&key_string);
        rows.push(BranchTreeRow::Group { key: key_string, label: label.into(), depth, expanded });
        if expanded {
            self.flatten(key, depth + 1, expand_all, collapsed, rows);
        }
    }
}

/// The tree's visible rows. `query` keeps the branches whose name contains it
/// (ignoring case) and expands every group so the matches show.
pub(crate) fn branch_tree_rows(
    branches: &[Branch],
    query: &str,
    collapsed: &HashSet<SharedString>,
) -> Vec<BranchTreeRow> {
    let query = query.trim().to_lowercase();
    let mut local = Node::default();
    let mut remote = Node::default();
    let mut rows = Vec::new();
    for branch in branches {
        let name = branch.name();
        // `origin/HEAD` only points at the remote's default branch.
        if branch.is_remote() && name.ends_with("/HEAD") {
            continue;
        }
        if branch.is_head && query.is_empty() {
            rows.push(BranchTreeRow::Head { ref_name: branch.ref_name.clone() });
        }
        if !query.is_empty() && !name.to_lowercase().contains(&query) {
            continue;
        }
        let root = if branch.is_remote() { &mut remote } else { &mut local };
        root.insert(name, &branch.ref_name, branch.is_head);
    }
    let expand_all = !query.is_empty();
    for (label, node) in [(LOCAL_GROUP, local), (REMOTE_GROUP, remote)] {
        if !node.is_empty() {
            node.flatten_group(label, label.to_string(), 0, expand_all, collapsed, &mut rows);
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use git::repository::Branch;
    use gpui::SharedString;
    use rustc_hash::FxHashSet as HashSet;

    use super::BranchTreeRow;
    use super::branch_tree_rows;

    fn branch(ref_name: &str, is_head: bool) -> Branch {
        Branch { is_head, ref_name: ref_name.to_string().into(), upstream: None, most_recent_commit: None }
    }

    fn branches() -> Vec<Branch> {
        vec![
            branch("refs/heads/sam/feature-x", false),
            branch("refs/heads/fix", false),
            branch("refs/heads/master", true),
            branch("refs/remotes/origin/HEAD", false),
            branch("refs/remotes/origin/master", false),
            branch("refs/remotes/origin/dependabot/npm", false),
        ]
    }

    fn describe(rows: &[BranchTreeRow]) -> Vec<String> {
        rows.iter()
            .map(|row| match row {
                BranchTreeRow::Head { ref_name } => format!("HEAD {ref_name}"),
                BranchTreeRow::Group { label, depth, expanded, .. } => {
                    format!("{}{} {label}", "  ".repeat(*depth), if *expanded { "v" } else { ">" })
                }
                BranchTreeRow::Branch { label, depth, is_head, .. } => {
                    format!("{}{label}{}", "  ".repeat(*depth), if *is_head { " *" } else { "" })
                }
            })
            .collect()
    }

    #[test]
    fn test_branch_tree_rows() {
        let rows = branch_tree_rows(&branches(), "", &HashSet::default());
        assert_eq!(
            describe(&rows),
            [
                "HEAD refs/heads/master",
                "v Local",
                "  master *",
                "  fix",
                "  v sam",
                "    feature-x",
                "v Remote",
                "  v origin",
                "    master",
                "    v dependabot",
                "      npm",
            ]
        );
        assert_eq!(
            rows[5],
            BranchTreeRow::Branch {
                ref_name: "refs/heads/sam/feature-x".into(),
                label: "feature-x".into(),
                depth: 2,
                is_head: false,
            }
        );
    }

    #[test]
    fn test_branch_tree_rows_collapsed() {
        let collapsed: HashSet<SharedString> = ["Local/sam".into(), "Remote".into()].into_iter().collect();
        let rows = branch_tree_rows(&branches(), "", &collapsed);
        assert_eq!(
            describe(&rows),
            ["HEAD refs/heads/master", "v Local", "  master *", "  fix", "  > sam", "> Remote"]
        );
    }

    #[test]
    fn test_branch_tree_rows_query() {
        let collapsed: HashSet<SharedString> = ["Remote".into()].into_iter().collect();
        let rows = branch_tree_rows(&branches(), " MAST ", &collapsed);
        assert_eq!(describe(&rows), ["v Local", "  master *", "v Remote", "  v origin", "    master"]);
        assert!(branch_tree_rows(&branches(), "nothing", &collapsed).is_empty());
        assert!(branch_tree_rows(&[], "", &collapsed).is_empty());
    }
}
