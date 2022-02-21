use git_shortcuts::{
    commit::{Author, CommitMessage, FeatMessage},
    error::Error,
    extract_from_branch,
};

use std::env::current_dir;

use clap::Parser;
use color_eyre::eyre::Result;
use git2::Repository;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct FeatArgs {
    /// Commit message.
    message: String,
    /// Whether this feature is a breaking change.
    #[clap(short, long)]
    breaking: bool,
    /// Whether to be verbose or not.
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = FeatArgs::parse();

    // NOTE: unsure how CWD can fail
    let repo = Repository::open(current_dir()?)?;
    let author = Author::try_from(&repo)?;

    let head = repo.head()?;

    let branch_name = head.shorthand().map_or(Err(Error::HeadIsNotABranch), Ok)?;
    let (team_name, issue_number) = extract_from_branch(branch_name)?;

    let message = FeatMessage {
        message: CommitMessage {
            team_name,
            issue_number,
            message: args.message,
        },
        breaking: args.breaking,
    };

    let tree_oid = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let parent_commit = head.resolve()?.peel_to_commit()?;

    let signature = &author.try_into()?;
    repo.commit(
        Some("HEAD"),
        signature,
        signature,
        &message.to_string(),
        &tree,
        &[&parent_commit],
    )?;

    if args.verbose {
        println!("{}", message);
    }

    Ok(())
}
