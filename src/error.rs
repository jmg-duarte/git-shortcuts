#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HEAD is not pointing to a branch.")]
    HeadIsNotABranch,
    #[error("{0} is not a known prefix.")]
    UnknownPrefix(String),
    #[error("{0} is not a valid branch name.")]
    InvalidBranchName(String),
    #[error("No staged changes were found.")]
    NoStagedChanges,
}
