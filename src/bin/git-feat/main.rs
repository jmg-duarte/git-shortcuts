use std::{
    env::current_dir,
    fmt::{Display, Write},
};

use clap::Parser;
use git2::{Commit, ReferenceType, Repository, Signature, Tree};
use phf::phf_map;
use regex::Regex;

const BREAKING_CHAR: char = '!';

static TEAM_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "ANALYTICS"
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Commit message.
    message: String,
    /// Whether this feature is a breaking change.
    #[clap(short, long)]
    breaking: bool,
}

/// General commit message.
#[derive(Debug)]
struct CommitMessage {
    team_name: String,
    issue_number: u32,
    message: String,
}

impl Display for CommitMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({}-{}): {}",
            self.team_name, self.issue_number, self.message
        ))
    }
}

/// Feature commit message.
#[derive(Debug)]
struct FeatMessage {
    message: CommitMessage,
    breaking: bool,
}

impl Display for FeatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("feat")?;
        if self.breaking {
            f.write_char(BREAKING_CHAR)?;
        }
        self.message.fmt(f)
    }
}

#[derive(Debug)]
struct Author {
    name: String,
    email: String,
}

impl TryFrom<&git2::Repository> for Author {
    type Error = git2::Error;

    fn try_from(repo: &git2::Repository) -> Result<Self, Self::Error> {
        Ok(Author {
            name: repo.config()?.get_string("user.name")?,
            email: repo.config()?.get_string("user.email")?,
        })
    }
}

impl<'a> TryInto<Signature<'a>> for Author {
    type Error = git2::Error;

    fn try_into(self) -> Result<Signature<'a>, Self::Error> {
        Signature::now(&self.name, &self.email)
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("HEAD is not pointing to a branch.")]
    HeadIsNotABranch,
    #[error("{0} is not a known prefix.")]
    UnknownPrefix(String),
    #[error("{0} is not a valid branch name.")]
    InvalidBranchName(String),
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let re = Regex::new("([A-Z])-?([0-9]+).*")?;
    let args = Args::parse();

    // NOTE: unsure how CWD can fail
    let cwd = current_dir()?;

    let repo = Repository::open(cwd)?;
    let author = Author::try_from(&repo)?;

    println!("{}", author);

    let head = repo.head()?;

    let branch_name = head.shorthand().map_or(Err(Error::HeadIsNotABranch), Ok)?;

    let captures = re
        .captures(branch_name)
        .map_or(Err(Error::InvalidBranchName(branch_name.to_string())), Ok)?;

    let team_name = TEAM_MAPPING
        .get(&captures[1])
        .map_or(Err(Error::UnknownPrefix(captures[1].to_string())), Ok)?;

    let issue_number = captures[2].parse::<u32>()?;

    let message = FeatMessage {
        message: CommitMessage {
            team_name: String::from(*team_name),
            issue_number,
            message: args.message,
        },
        breaking: args.breaking,
    };

    let tree_oid = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let parent_commit = head.resolve()?.peel_to_commit()?;

    let signature = &author.try_into()?;
    let _ = repo.commit(
        Some("HEAD"),
        signature,
        signature,
        &message.to_string(),
        &tree,
        &[&parent_commit],
    )?;

    println!("{}", message);
    Ok(())
}
