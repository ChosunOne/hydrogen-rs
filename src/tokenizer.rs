use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("{0} is not a keyword")]
    KeywordError(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
}

#[derive(Debug, Clone)]
pub(crate) enum Token {
    IntLit(String),
    Keyword(Keywords),
}

#[derive(Debug, Clone)]
pub(crate) enum Keywords {
    Exit,
    Semi,
}

impl TryFrom<&str> for Keywords {
    type Error = TokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "exit" => Ok(Self::Exit),
            ";" => Ok(Self::Semi),
            _ => Err(TokenError::KeywordError(value.into())),
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
        Err(TokenError::InvalidToken(val.into()))
    }
}

impl From<Keywords> for Token {
    fn from(value: Keywords) -> Self {
        Self::Keyword(value)
    }
}

pub fn split_with_char<'a>(strings: &'a [&'a str], character: &'a str) -> Vec<&'a str> {
    let mut new_vec = Vec::<&str>::new();
    for s in strings {
        let mut start = 0;
        while let Some(pos) = s[start..].find(character) {
            let actual_pos = start + pos;
            new_vec.push(s[start..actual_pos].into());
            new_vec.push(character);
            start = actual_pos + 1;
        }

        if start < s.len() {
            new_vec.push(s[start..].into());
        }
    }
    new_vec
}

pub(crate) fn tokenize_source(src: &str) -> Result<Vec<Token>, TokenError> {
    let split_contents = src.split_whitespace().collect::<Vec<_>>();
    let split_with_semi_contents = split_with_char(&split_contents, ";");
    let tokens = split_with_semi_contents
        .into_iter()
        .map(Token::try_from)
        .collect::<Vec<_>>();
    tokens.into_iter().collect()
}
