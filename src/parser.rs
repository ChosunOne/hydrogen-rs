use crate::tokenizer::{Keywords, Token};
use std::slice::Iter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse tokens")]
    ParseError,
}

#[derive(Clone, Debug)]
pub(crate) struct NodeExpr {
    pub(crate) value: String,
}

#[derive(Clone, Debug)]
pub(crate) struct NodeExit {
    pub(crate) expr: NodeExpr,
}

fn parse_expr(iter: &mut Iter<'_, Token>) -> Result<NodeExpr, ParseError> {
    match iter.next() {
        Some(Token::IntLit(v)) => Ok(NodeExpr {
            value: v.to_owned(),
        }),
        _ => Err(ParseError::ParseError),
    }
}

pub(crate) fn parse<'a>(tokens: impl Into<Iter<'a, Token>>) -> Result<NodeExit, ParseError> {
    let mut iter = tokens.into();
    let mut exit_node = None;
    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(kw) => match kw {
                Keywords::Exit => {
                    let expr = parse_expr(&mut iter)?;
                    exit_node = Some(NodeExit { expr });
                    match iter.next() {
                        Some(Token::Keyword(Keywords::Semi)) => {}
                        _ => return Err(ParseError::ParseError),
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
    let exit_node = exit_node.ok_or(ParseError::ParseError)?;
    Ok(exit_node)
}
