use crate::{
    constants::*,
    tokenizer::{Keywords, Operators, Token},
};
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
pub(crate) enum NodeTerm {
    IntLit(Token),
    Ident(Token),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeExpr {
    Term(NodeTerm),
    BinExpr(NodeBinExpr),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BinExpr {
    pub(crate) lhs: Box<NodeExpr>,
    pub(crate) rhs: Box<NodeExpr>,
    pub(crate) precedence: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeBinExpr {
    Add(BinExpr),
    Multiply(BinExpr),
}

fn parse_bin_expr(iter: &mut Iter<'_, Token>) -> Result<Option<NodeBinExpr>, ParseError> {
    let lhs = Box::new(NodeExpr::Term(parse_term(iter)?));
    let op = match iter.next() {
        Some(Token::Keyword(Keywords::Operator(op))) => op,
        Some(Token::Keyword(kw)) => match kw {
            Keywords::Semi
            | Keywords::Assign
            | Keywords::OpenParenthesis
            | Keywords::CloseParenthesis => return Ok(None),
            _ => {
                return Err(ParseError::Parse(
                    format!("Unexpected keyword: {:?}", kw).into(),
                ))
            }
        },
        Some(t) => return Err(ParseError::Parse(format!("Unexpected token: {t:?}"))),
        None => return Ok(None),
    };

    let rhs = Box::new(parse_expr(iter)?);
    match op {
        Operators::Plus => Ok(Some(NodeBinExpr::Add(BinExpr {
            lhs,
            rhs,
            precedence: 0,
        }))),
    }
}

fn parse_expr(iter: &mut Iter<'_, Token>) -> Result<NodeExpr, ParseError> {
    let mut bin_expr_iter = iter.clone();
    if let Some(node_bin_expr) = parse_bin_expr(&mut bin_expr_iter)? {
        *iter = bin_expr_iter;
        return Ok(NodeExpr::BinExpr(node_bin_expr));
    }
    Ok(NodeExpr::Term(parse_term(iter)?))
}

fn parse_term(iter: &mut Iter<'_, Token>) -> Result<NodeTerm, ParseError> {
    match iter.next() {
        Some(v @ Token::IntLit(_)) => Ok(NodeTerm::IntLit(v.clone())),
        Some(v @ Token::Ident(_)) => Ok(NodeTerm::Ident(v.clone())),
        _ => Err(ParseError::ExpectedToken("Identifier or literal".into())),
    }
}

fn parse_exit(iter: &mut Iter<'_, Token>) -> Result<NodeStatement, ParseError> {
    match iter.next() {
        Some(Token::Keyword(Keywords::OpenParenthesis)) => {}
        _ => return Err(ParseError::ExpectedToken(KW_OPEN_PARENTHESIS.into())),
    }
    let expr = parse_expr(iter)?;
    match iter.next() {
        Some(Token::Keyword(Keywords::CloseParenthesis)) => {}
        _ => return Err(ParseError::ExpectedToken(KW_CLOSE_PARENTHESIS.into())),
    }
    match iter.next() {
        Some(Token::Keyword(Keywords::Semi)) => {}
        _ => return Err(ParseError::ExpectedToken(KW_SEMI.into())),
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
        _ => return Err(ParseError::ExpectedToken(KW_ASSIGN.into())),
    }

    let expr = parse_expr(iter)?;
    match iter.next() {
        Some(Token::Keyword(Keywords::Semi)) => {}
        _ => return Err(ParseError::ExpectedToken(KW_SEMI.into())),
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
