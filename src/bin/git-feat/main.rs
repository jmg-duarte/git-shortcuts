use std::env::current_dir;

use clap::Parser;
use git2::Repository;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Commit message.
    message: String,
    /// Whether this feature is a breaking change.
    #[clap(short, long)]
    breaking: bool,
}

fn main() {
    let args = Args::parse();
    let cwd = current_dir().unwrap();
    let repo = Repository::open(cwd).unwrap();
    println!("{:#?}", repo.head().unwrap().name_bytes())
}