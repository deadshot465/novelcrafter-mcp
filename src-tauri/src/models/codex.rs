use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Codex {
    pub codex_type: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub tags: Vec<String>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    NoFrontmatter,
    InvalidFrontmatter(String),
    MissingRequiredField(&'static str),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NoFrontmatter => {
                write!(f, "No frontmatter found in markdown file")
            }
            ParseError::InvalidFrontmatter(msg) => {
                write!(f, "Invalid frontmatter: {}", msg)
            }
            ParseError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
        }
    }
}

impl Error for ParseError {}
