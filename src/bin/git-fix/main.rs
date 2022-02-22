use git_shortcuts::{
    check_staged_changes, commit,
    commit::{CommitMessage, FixMessage},
    error::Error,
    extract_from_branch,
};

use std::env::current_dir;

use clap::Parser;
use color_eyre::eyre::Result;
use git2::Repository;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct FixArgs {
    /// Commit message.
    message: String,
    /// Silence messages.
    #[clap(short, long)]
    quiet: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = FixArgs::parse();

    // NOTE: unsure how CWD can fail
    let repo = Repository::open(current_dir()?)?;

    let head = repo.head()?;
    let branch_name = head.shorthand().map_or(Err(Error::HeadIsNotABranch), Ok)?;
    let (team_name, issue_number) = extract_from_branch(branch_name)?;

    let message = FixMessage {
        message: CommitMessage {
            team_name,
            issue_number,
            message: args.message,
        },
    };

    // Check for staged changes
    check_staged_changes(&repo)?;
    // Commit changes
    commit(&repo, &message.to_string())?;

    if !args.quiet {
        println!("{}", message);
    }

    Ok(())
}
