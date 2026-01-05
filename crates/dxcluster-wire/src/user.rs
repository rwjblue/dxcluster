use dxcluster_model::Spot;

use crate::error::UserParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum UserCommand {
    Dx { target: String },
    ShDx,
    Raw(String),
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

    if let Some(rest) = trimmed.strip_prefix("DX ") {
        return Ok(UserCommand::Dx {
            target: rest.trim().to_string(),
        });
    }

    if trimmed.eq_ignore_ascii_case("SH/DX") {
        return Ok(UserCommand::ShDx);
    }

    Ok(UserCommand::Raw(trimmed.to_string()))
}

pub fn format_banner(node_name: &str) -> String {
    format!("Welcome to {node_name} DX cluster")
}

pub fn format_prompt() -> String {
    String::from(">")
}
