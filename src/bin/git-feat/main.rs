use git_shortcuts::{
    check_staged_changes, commit,
    commit::{CommitMessage, FeatMessage},
    config::{default_config_path, BranchInfo, Config},
    error::Error,
};

use std::{env::current_dir, fs};

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
    /// Silence messages.
    #[clap(short, long)]
    quiet: bool,
    /// Path to custom config file.
    /// The default value is "$HOME/.config/.git-shortcuts.toml" (*nix)
    #[clap(short, long, default_value_t=default_config_path())]
    config: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = FeatArgs::parse();
    println!("{}", args.config);

    let file = fs::read_to_string(args.config)?;
    let config = toml::from_str::<Config>(&file)?;

    // NOTE: unsure how CWD can fail
    let repo = Repository::open(current_dir()?)?;

    let head = repo.head()?;
    let branch_name = head.shorthand().map_or(Err(Error::HeadIsNotABranch), Ok)?;
    let BranchInfo(team_name, issue_number) = config.extract_branch_info(branch_name)?;

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

    if !args.quiet {
        println!("{}", message);
    }

    Ok(())
}
