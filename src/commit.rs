use std::fmt::{Display, Write};

const BREAKING_CHAR: char = '!';

/// Commit author.
#[derive(Debug)]
pub struct Author {
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

impl<'a> TryInto<git2::Signature<'a>> for Author {
    type Error = git2::Error;

    fn try_into(self) -> Result<git2::Signature<'a>, Self::Error> {
        git2::Signature::now(&self.name, &self.email)
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

/// General commit message.
#[derive(Debug)]
pub struct CommitMessage {
    pub team_name: String,
    pub issue_number: u32,
    pub message: String,
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
pub struct FeatMessage {
    pub message: CommitMessage,
    pub breaking: bool,
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

/// Fix commit message.
#[derive(Debug)]
pub struct FixMessage {
    pub message: CommitMessage,
}

impl Display for FixMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("fix")?;
        self.message.fmt(f)
    }
}
