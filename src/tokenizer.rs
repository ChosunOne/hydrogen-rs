use crate::constants::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("{0} is not a keyword")]
    Keyword(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    IntLit(String),
    Ident(String),
    Keyword(Keywords),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Self::IntLit(s) => s.clone(),
            Self::Ident(s) => s.clone(),
            Self::Keyword(kw) => format!("{kw:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Keywords {
    Exit,
    Semi,
    OpenParenthesis,
    CloseParenthesis,
    Let,
    Assign,
}

impl TryFrom<&str> for Keywords {
    type Error = TokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            KW_EXIT => Ok(Self::Exit),
            KW_SEMI => Ok(Self::Semi),
            KW_OPEN_PARENTHESIS => Ok(Self::OpenParenthesis),
            KW_CLOSE_PARENTHESIS => Ok(Self::CloseParenthesis),
            KW_LET => Ok(Self::Let),
            KW_ASSIGN => Ok(Self::Assign),
            _ => Err(TokenError::Keyword(value.into())),
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = TokenError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        if let Ok(keyword) = Keywords::try_from(val) {
            return Ok(keyword.into());
        }
        if val.chars().all(|c| char::is_digit(c, 10)) {
            return Ok(Token::IntLit(val.into()));
        }
        Ok(Token::Ident(val.into()))
    }
}

impl From<Keywords> for Token {
    fn from(value: Keywords) -> Self {
        Self::Keyword(value)
    }
}

pub fn split_with_char(strings: &[String], character: &'static str) -> Vec<String> {
    let mut new_vec = Vec::<String>::new();
    for s in strings {
        let mut start = 0;
        while let Some(pos) = s[start..].find(character) {
            let actual_pos = start + pos;
            new_vec.push(s[start..actual_pos].into());
            new_vec.push(character.into());
            start = actual_pos + 1;
        }

        if start < s.len() {
            new_vec.push(s[start..].into());
        }
    }
    new_vec
}

fn split_tokens(src: &str) -> Vec<String> {
    let mut split_src = src
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>();

    for kw in TOKENIZER_KEYWORDS {
        split_src = split_with_char(&split_src, kw);
    }

    split_src
        .into_iter()
        .filter(|s| !str::is_empty(s))
        .collect()
}

pub(crate) fn tokenize_source(src: &str) -> Result<Vec<Token>, TokenError> {
    let split_src = split_tokens(src);
    let tokens = split_src
        .into_iter()
        .map(|s| Token::try_from(s.as_str()))
        .collect::<Vec<_>>();
    tokens.into_iter().collect()
}
