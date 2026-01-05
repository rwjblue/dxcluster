//! Parsers and formatters for user-facing commands and responses.

use dxcluster_model::Spot;
use dxcluster_types::{Callsign, FrequencyHz, normalize};

use crate::error::UserParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum UserCommand {
    Dx {
        dx: Callsign,
        frequency: FrequencyHz,
        comment: String,
    },
    Show(ShowCommand),
    Heartbeat,
    Raw(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShowCommand {
    Dx,
    Filters,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerLine {
    Banner(String),
    Prompt,
    Spot(Spot),
    Message(String),
}

pub fn parse_line(line: &str) -> Result<UserCommand, UserParseError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(UserParseError::Empty);
    }

    if trimmed.eq_ignore_ascii_case("PING") || trimmed.eq_ignore_ascii_case("HEARTBEAT") {
        return Ok(UserCommand::Heartbeat);
    }

    if trimmed.len() >= 2 && trimmed[..2].eq_ignore_ascii_case("DX") {
        return parse_dx_command(&trimmed[2..]);
    }

    if trimmed.eq_ignore_ascii_case("SH/DX") {
        return Ok(UserCommand::Show(ShowCommand::Dx));
    }

    if trimmed.eq_ignore_ascii_case("SH/FILTERS") || trimmed.eq_ignore_ascii_case("SHOW/FILTERS") {
        return Ok(UserCommand::Show(ShowCommand::Filters));
    }

    Ok(UserCommand::Raw(trimmed.to_string()))
}

pub fn format_banner(node_name: &str) -> String {
    format!("Welcome to {node_name} DX cluster")
}

pub fn format_prompt() -> String {
    String::from(">")
}

pub fn format_command(cmd: &UserCommand) -> String {
    match cmd {
        UserCommand::Dx {
            dx,
            frequency,
            comment,
        } => format!("DX {dx} {} {comment}", frequency.to_khz_string()),
        UserCommand::Show(ShowCommand::Dx) => String::from("SH/DX"),
        UserCommand::Show(ShowCommand::Filters) => String::from("SH/FILTERS"),
        UserCommand::Heartbeat => String::from("PING"),
        UserCommand::Raw(raw) => raw.to_string(),
    }
}

fn parse_dx_command(rest: &str) -> Result<UserCommand, UserParseError> {
    let mut tokens = rest.split_whitespace();
    let Some(dx) = tokens.next() else {
        return Err(UserParseError::MissingCallsign);
    };
    let Some(freq) = tokens.next() else {
        return Err(UserParseError::MissingFrequency);
    };
    let comment = tokens.collect::<Vec<_>>().join(" ");

    let dx = Callsign::parse_loose(dx).map_err(UserParseError::InvalidCallsign)?;
    let frequency = FrequencyHz::from_khz_str(freq).map_err(UserParseError::InvalidFrequency)?;
    let comment = normalize::comment(&comment);

    Ok(UserCommand::Dx {
        dx,
        frequency,
        comment,
    })
}
