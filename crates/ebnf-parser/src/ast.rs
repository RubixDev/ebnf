use crate::{
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Syntax<'src> {
    pub span: Span,
    pub rules: Vec<SyntaxRule<'src>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntaxRule<'src> {
    pub span: Span,
    pub name: &'src str,
    pub definitions: Vec<SingleDefinition<'src>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SingleDefinition<'src> {
    pub span: Span,
    pub terms: Vec<SyntacticTerm<'src>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntacticTerm<'src> {
    pub span: Span,
    pub factor: SyntacticFactor<'src>,
    pub exception: Option<SyntacticException<'src>>,
}

pub type SyntacticException<'src> = SyntacticFactor<'src>;

#[derive(Debug, PartialEq, Clone)]
pub struct SyntacticFactor<'src> {
    pub span: Span,
    pub repetition: Option<usize>,
    pub primary: SyntacticPrimary<'src>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntacticPrimary<'src> {
    pub span: Span,
    pub kind: SyntacticPrimaryKind<'src>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntacticPrimaryKind<'src> {
    OptionalSequence(Vec<SingleDefinition<'src>>),
    RepeatedSequence(Vec<SingleDefinition<'src>>),
    GroupedSequence(Vec<SingleDefinition<'src>>),
    MetaIdentifier(&'src str),
    TerminalString(&'src str),
    SpecialSequence(&'src str),
    EmptySequence,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comment<'src> {
    pub span: Span,
    pub text: &'src str,
}

impl<'src> TryFrom<Token<'src>> for Comment<'src> {
    type Error = &'static str;

    fn try_from(value: Token<'src>) -> Result<Self, Self::Error> {
        match value.kind {
            TokenKind::Comment(text) => Ok(Comment { span: value.span, text }),
            _ => Err("Comment node can only be constructed from Comment TokenKind"),
        }
    }
}
