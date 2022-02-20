use std::{
    env::current_dir,
    fmt::{Display, Write},
};

use clap::Parser;
use git2::Repository;
use regex::Regex;
use phf::phf_map;

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

fn main() {
    let re = Regex::new("([A-Z])-?([0-9]+).*").unwrap();

    let args = Args::parse();

    // NOTE: unsure how CWD can fail
    let cwd = current_dir().unwrap();

    let repo = Repository::open(cwd).unwrap();
    let head = repo.head().unwrap();

    // TODO do not panic, print error and exit
    if !head.is_branch() {
        panic!("not a branch");
    }
    let head_name = head.name().unwrap();

    // TODO: check if the string matches
    let captures = re.captures(head_name).unwrap();

    // TODO: give a decent error message
    let team_name = TEAM_MAPPING.get(&captures[1]).unwrap();

    let issue_number = captures[2].parse::<u32>().unwrap();

    let message = FeatMessage {
        message: CommitMessage {
            team_name: String::from(*team_name),
            issue_number,
            message: args.message,
        },
        breaking: args.breaking,
    };

    println!("{}", message);
}
