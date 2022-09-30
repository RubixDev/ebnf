use std::fmt::{Debug, Display};

use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind<'src> {
    Identifier(&'src str),
    Terminal(&'src str),
    Comment(&'src str),
    SpecialSeq(&'src str),
    Integer(usize),

    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Pipe,
    Comma,
    Semicolon,
    Equal,
    Star,
    Dash,
}

impl Display for TokenKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier(name) => write!(f, "{name}"),
            TokenKind::Terminal(text) => write!(f, "\"{text}\""),
            TokenKind::Comment(text) => write!(f, "(* {text} *)"),
            TokenKind::SpecialSeq(text) => write!(f, "? {text} ?"),
            TokenKind::Integer(num) => write!(f, "{num}"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Dash => write!(f, "-"),
        }
    }
}

/// A token with positional information
#[derive(Clone)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Span,
}

impl<'src> Token<'src> {
    pub(crate) fn new(kind: TokenKind<'src>, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} @ {:?})", self.kind, self.span)
    }
}
