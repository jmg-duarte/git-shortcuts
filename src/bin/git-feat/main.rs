use git_shortcuts::{
    check_staged_changes, commit,
    commit::{CommitMessage, FeatMessage},
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
    // Check for staged changes
    check_staged_changes(&repo)?;
    // Commit changes
    commit(&repo, &message.to_string())?;

    if args.verbose {
        println!("{}", message);
    }

    Ok(())
}
