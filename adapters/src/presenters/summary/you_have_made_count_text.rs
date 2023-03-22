use crate::presenters::process_plural::process_plural;

pub(super) fn you_have_made_count_text(commit_count: &u32, repo_count: &u32) -> String {
    let commit_plural_text = process_plural(commit_count, "commit", "commits");
    let repo_plural_text = process_plural(repo_count, "repository", "repositories");
    format!(
        "You have made *{}* on *{}*",
        commit_plural_text, repo_plural_text
    )
}
