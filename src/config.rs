use std::{collections::HashMap, fs, path::Path};

use color_eyre::eyre::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::error::Error;

lazy_static::lazy_static! {
    // safety: if this fails, the program is useless anyway
    pub static ref BRANCH_NAME_REGEX: Regex = Regex::new("([A-Z])-?([0-9]+).*").unwrap();
}

/// Returns the default path for the configuration file.
// NOTE(duarte): does not work for Windows
pub fn default_config_path() -> String {
    let home = std::env::var("HOME").unwrap();
    return home + "/.config/git-shortcuts.toml";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    prefixes: HashMap<String, String>,
}

impl Config {
    /// Read a configuration file from a given filename.
    pub fn from_filename<P: AsRef<Path>>(filename: P) -> Self {
        let file = fs::read_to_string(filename).unwrap();
        let mut config = toml::from_str::<Config>(&file).unwrap();

        let mut reversed_prefixes = HashMap::with_capacity(config.prefixes.capacity());
        for (k, v) in config.prefixes {
            reversed_prefixes.insert(k, v);
        }

        config.prefixes = reversed_prefixes;
        config
    }

    /// Extract the branch info using the configured prefixes.
    pub fn extract_branch_info(&self, branch_name: &str) -> Result<BranchInfo> {
        let captures = BRANCH_NAME_REGEX
            .captures(branch_name)
            .map_or(Err(Error::InvalidBranchName(branch_name.to_string())), Ok)?;

        let team_name = self
            .prefixes
            .get(&captures[1])
            .map_or(Err(Error::UnknownPrefix(captures[1].to_string())), Ok)?;

        let issue_number = captures[2].parse::<u32>()?;

        Ok(BranchInfo(team_name.to_owned(), issue_number))
    }
}

pub struct BranchInfo(pub String, pub u32);
