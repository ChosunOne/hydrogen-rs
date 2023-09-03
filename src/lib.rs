use std::char;

#[derive(Debug)]
pub enum Token {
    Return,
    IntLit(Option<String>),
    Semi,
}

impl TryFrom<&str> for Token {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let first_char = val.chars().next().unwrap();

        match (char::is_alphabetic(first_char), val) {
            (true, "return") => Ok(Token::Return),
            (true, invalid_token) => Err(format!("Invalid token: {invalid_token}")),
            (false, ";") => Ok(Token::Semi),
            (false, val) => match (val.chars().all(|c| char::is_digit(c, 10)), val) {
                (true, number_val) => Ok(Token::IntLit(Some(number_val.into()))),
                (false, invalid_token) => Err(format!("Invalid token: {invalid_token}")),
            },
        }
    }
}

pub fn split_with_char<'a, 'b>(strings: &'a [&'a str], character: &'a str) -> Vec<&'a str> {
    let mut new_vec = Vec::<&str>::new();
    for s in strings.iter() {
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
