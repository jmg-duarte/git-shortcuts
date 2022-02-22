pub mod commit;
pub mod config;
pub mod error;

use color_eyre::eyre::Result;
use git2::Repository;
use regex::Regex;

use crate::commit::Author;
use crate::error::Error;

lazy_static::lazy_static! {
    // safety: if this fails, the program is useless anyway
    pub static ref BRANCH_NAME_REGEX: Regex = Regex::new("([A-Z])-?([0-9]+).*").unwrap();
}

pub fn check_staged_changes(repository: &Repository) -> Result<()> {
    let mut has_staged = false;
    for status_entry in repository.statuses(None)?.iter() {
        let status = status_entry.status();
        has_staged |= status.is_index_deleted()
            || status.is_index_modified()
            || status.is_index_new()
            || status.is_index_renamed()
            || status.is_index_typechange();
        if has_staged {
            return Ok(());
        }
    }
    Err(Error::NoStagedChanges)?
}

pub fn commit(repository: &Repository, message: &str) -> Result<()> {
    let tree_oid = repository.index()?.write_tree()?;
    let tree = repository.find_tree(tree_oid)?;
    let parent_commit = repository.head()?.resolve()?.peel_to_commit()?;
    let author = Author::try_from(repository)?;
    let signature = &author.try_into()?;
    repository.commit(
        Some("HEAD"),
        signature,
        signature,
        &message.to_string(),
        &tree,
        &[&parent_commit],
    )?;
    Ok(())
}
