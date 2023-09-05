use crate::tokenizer::{Keywords, Token};
use std::slice::Iter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse tokens: {0}")]
    Parse(String),
    #[error("Expected: {0}")]
    ExpectedToken(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeExpr {
    IntLit(Token),
    Ident(Token),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeStatement {
    Let((Token, NodeExpr)),
    Exit(NodeExpr),
    End,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NodeProgram {
    pub(crate) statements: Vec<NodeStatement>,
}

fn parse_expr(iter: &mut Iter<'_, Token>) -> Result<NodeExpr, ParseError> {
    match iter.next() {
        Some(v @ Token::IntLit(_)) => Ok(NodeExpr::IntLit(v.clone())),
        Some(v @ Token::Ident(_)) => Ok(NodeExpr::Ident(v.clone())),
        _ => Err(ParseError::ExpectedToken("IntLit".into())),
    }
}

fn parse_exit(iter: &mut Iter<'_, Token>) -> Result<NodeStatement, ParseError> {
    match iter.next() {
        Some(Token::Keyword(Keywords::OpenParenthesis)) => {}
        _ => return Err(ParseError::ExpectedToken("(".into())),
    }
    let expr = parse_expr(iter)?;
    match iter.next() {
        Some(Token::Keyword(Keywords::CloseParenthesis)) => {}
        _ => return Err(ParseError::ExpectedToken(")".into())),
    }
    match iter.next() {
        Some(Token::Keyword(Keywords::Semi)) => {}
        _ => return Err(ParseError::ExpectedToken(";".into())),
    }

    Ok(NodeStatement::Exit(expr))
}

fn parse_let(iter: &mut Iter<'_, Token>) -> Result<NodeStatement, ParseError> {
    let ident = match iter.next() {
        Some(i @ Token::Ident(_)) => i.clone(),
        _ => return Err(ParseError::ExpectedToken("Ident".into())),
    };
    match iter.next() {
        Some(Token::Keyword(Keywords::Assign)) => {}
        _ => return Err(ParseError::ExpectedToken("=".into())),
    }

    let expr = parse_expr(iter)?;
    match iter.next() {
        Some(Token::Keyword(Keywords::Semi)) => {}
        _ => return Err(ParseError::ExpectedToken(";".into())),
    }
    Ok(NodeStatement::Let((ident, expr)))
}

pub(crate) fn parse_statement(iter: &mut Iter<'_, Token>) -> Result<NodeStatement, ParseError> {
    if let Some(token) = iter.next() {
        match token {
            Token::Keyword(kw) => match kw {
                Keywords::Exit => return parse_exit(iter),
                Keywords::Let => return parse_let(iter),
                _ => return Err(ParseError::Parse(format!("Unknown Keyword: {kw:?}"))),
            },
            _ => return Err(ParseError::Parse(format!("Unknown Token: {token:?}"))),
        }
    }
    Ok(NodeStatement::End)
}

pub(crate) fn parse_program(iter: &mut Iter<'_, Token>) -> Result<NodeProgram, ParseError> {
    let mut statements = Vec::new();
    loop {
        match parse_statement(iter) {
            Ok(s) => {
                statements.push(s.clone());
                if s == NodeStatement::End {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(NodeProgram { statements })
}
