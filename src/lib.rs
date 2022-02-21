pub mod commit;
pub mod error;

use color_eyre::eyre::Result;
use phf::phf_map;
use regex::Regex;

use crate::error::Error;

lazy_static::lazy_static! {
    // safety: if this fails, the program is useless anyway
    pub static ref BRANCH_NAME_REGEX: Regex = Regex::new("([A-Z])-?([0-9]+).*").unwrap();
}

// TODO: convert this into a dynamic map, read from a config file
pub static TEAM_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "ANALYTICS"
};

pub fn extract_from_branch(branch_name: &str) -> Result<(String, u32)> {
    let captures = BRANCH_NAME_REGEX
        .captures(branch_name)
        .map_or(Err(Error::InvalidBranchName(branch_name.to_string())), Ok)?;

    let team_name = TEAM_MAPPING
        .get(&captures[1])
        .map_or(Err(Error::UnknownPrefix(captures[1].to_string())), Ok)?;

    let issue_number = captures[2].parse::<u32>()?;

    Ok((String::from(*team_name), issue_number))
}
