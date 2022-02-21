pub mod commit;
pub mod error;

use color_eyre::eyre::Result;
use git2::Repository;
use phf::phf_map;
use regex::Regex;

use crate::commit::Author;
use crate::error::Error;

lazy_static::lazy_static! {
    // safety: if this fails, the program is useless anyway
    pub static ref BRANCH_NAME_REGEX: Regex = Regex::new("([A-Z])-?([0-9]+).*").unwrap();
}

// TODO: convert this into a dynamic map, read from a config file
pub static TEAM_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "ANALYTICS"
};

pub fn extract_from_branch(branch_name: &str) -> Result<(String, u32)> {
    let captures = BRANCH_NAME_REGEX
        .captures(branch_name)
        .map_or(Err(Error::InvalidBranchName(branch_name.to_string())), Ok)?;

    let team_name = TEAM_MAPPING
        .get(&captures[1])
        .map_or(Err(Error::UnknownPrefix(captures[1].to_string())), Ok)?;

    let issue_number = captures[2].parse::<u32>()?;

    Ok((String::from(*team_name), issue_number))
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
