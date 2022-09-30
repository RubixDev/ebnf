use std::{collections::HashMap, mem};

use crate::{
    ast::*,
    error::SyntaxError,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
};

pub type CommentMap<'src> = HashMap<usize, Vec<Comment<'src>>>;

#[derive(Debug, Clone)]
pub struct ParseResult<'src> {
    pub comments: CommentMap<'src>,
    pub tokens: Vec<Token<'src>>,
    pub syntax: Syntax<'src>,
}

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    prev_tok: Option<Token<'src>>,
    curr_tok: Option<Token<'src>>,
    prev_span: Span,
    curr_span: Span,
    tokens: Vec<Token<'src>>,
    comments: CommentMap<'src>,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self {
            lexer,
            prev_tok: None,
            curr_tok: None,
            prev_span: Span::new(0, 1),
            curr_span: Span::new(0, 1),
            tokens: vec![],
            comments: HashMap::new(),
        }
    }

    pub fn parse(mut self) -> Result<ParseResult<'src>, SyntaxError> {
        self.next()?;
        let syntax = self.syntax()?;
        self.next()?; // add prev_tok to tokens list

        if let Some(curr_tok) = &self.curr_tok {
            return Err(SyntaxError::new(curr_tok.span, "Expected EOF".into()));
        }
        Ok(ParseResult {
            comments: self.comments,
            tokens: self.tokens,
            syntax,
        })
    }

    fn next(&mut self) -> Result<(), SyntaxError> {
        if let Some(prev_tok) = self.prev_tok.take() {
            self.tokens.push(prev_tok);
        }

        self.prev_tok = self.curr_tok.take();
        self.curr_tok = self.lexer.next_token()?;

        let mut comments = vec![];
        while let Some(Token {
            kind: TokenKind::Comment(_),
            ..
        }) = self.curr_tok
        {
            let comment = self.curr_tok.take().expect("`curr_tok` is a comment token");
            comments.push(Comment::try_from(comment).expect("`comment` is a comment token"));
            self.curr_tok = self.lexer.next_token()?;
        }
        if !comments.is_empty() {
            self.comments.insert(
                self.tokens.len() + self.prev_tok.is_some() as usize,
                comments,
            );
        }

        mem::swap(&mut self.prev_span, &mut self.curr_span);
        if let Some(curr_tok) = &self.curr_tok {
            self.curr_span = curr_tok.span;
        } else {
            self.curr_span = Span::new(self.lexer.index, self.lexer.index + 1);
        }
        Ok(())
    }

    fn is_kind(&mut self, kind: TokenKind) -> Result<bool, SyntaxError> {
        Ok(matches!(self.curr_tok, Some(Token { kind: tok_kind, .. }) if tok_kind == kind))
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), SyntaxError> {
        match self.curr_tok.as_ref().map(|tok| &tok.kind) {
            Some(tok_kind) if tok_kind == &kind => {
                self.next()?;
                Ok(())
            }
            Some(tok_kind) => Err(SyntaxError::new(
                self.curr_span,
                format!("Expected '{kind}', was '{tok_kind}'").into(),
            )),
            None => Err(SyntaxError::new(
                self.curr_span,
                format!("Expected '{kind}'").into(),
            )),
        }
    }

    fn syntax(&mut self) -> Result<Syntax<'src>, SyntaxError> {
        let start = self.curr_span.start;
        let mut rules = vec![];

        while self.curr_tok.is_some() {
            rules.push(self.syntax_rule()?);
        }
        if rules.is_empty() {
            return Err(SyntaxError::new(
                self.curr_span,
                "Syntax requires at least on syntax rule".into(),
            ));
        }

        Ok(Syntax {
            span: Span::new(start, self.prev_span.end),
            rules,
        })
    }

    fn syntax_rule(&mut self) -> Result<SyntaxRule<'src>, SyntaxError> {
        let start = self.curr_span.start;

        let name = match self.curr_tok {
            Some(Token {
                kind: TokenKind::Identifier(name),
                ..
            }) => name,
            _ => {
                return Err(SyntaxError::new(
                    self.curr_span,
                    "Expected identifier".into(),
                ))
            }
        };
        self.next()?;

        self.expect(TokenKind::Equal)?;
        let definitions = self.definitions_list()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(SyntaxRule {
            span: Span::new(start, self.prev_span.end),
            name,
            definitions,
        })
    }

    fn definitions_list(&mut self) -> Result<Vec<SingleDefinition<'src>>, SyntaxError> {
        let mut definitions = vec![self.single_definition()?];

        while self.is_kind(TokenKind::Pipe)? {
            self.next()?;
            definitions.push(self.single_definition()?);
        }

        Ok(definitions)
    }

    fn single_definition(&mut self) -> Result<SingleDefinition<'src>, SyntaxError> {
        let start = self.curr_span.start;
        let mut terms = vec![self.syntactic_term()?];

        while self.is_kind(TokenKind::Comma)? {
            self.next()?;
            terms.push(self.syntactic_term()?);
        }

        Ok(SingleDefinition {
            span: Span::new(start, self.prev_span.end),
            terms,
        })
    }

    fn syntactic_term(&mut self) -> Result<SyntacticTerm<'src>, SyntaxError> {
        let start = self.curr_span.start;
        let factor = self.syntactic_factor()?;
        let exception = match self.is_kind(TokenKind::Dash)? {
            true => {
                self.next()?;
                Some(self.syntactic_exception()?)
            }
            false => None,
        };

        Ok(SyntacticTerm {
            span: Span::new(start, self.prev_span.end),
            factor,
            exception,
        })
    }

    #[inline]
    fn syntactic_exception(&mut self) -> Result<SyntacticException<'src>, SyntaxError> {
        self.syntactic_factor()
    }

    fn syntactic_factor(&mut self) -> Result<SyntacticFactor<'src>, SyntaxError> {
        let start = self.curr_span.start;
        let repetition = match self.curr_tok {
            Some(Token {
                kind: TokenKind::Integer(num),
                ..
            }) => {
                self.next()?;
                self.expect(TokenKind::Star)?;
                Some(num)
            }
            _ => None,
        };
        let primary = self.syntactic_primary()?;

        Ok(SyntacticFactor {
            span: Span::new(start, self.prev_span.end),
            repetition,
            primary,
        })
    }

    fn syntactic_primary(&mut self) -> Result<SyntacticPrimary<'src>, SyntaxError> {
        let start = self.curr_span.start;
        let kind = match self
            .curr_tok
            .as_ref()
            .map_or(TokenKind::Semicolon, |tok| tok.kind)
        {
            TokenKind::LBracket => SyntacticPrimaryKind::OptionalSequence(
                self.delimited_definitions_list(TokenKind::RBracket)?,
            ),
            TokenKind::LBrace => SyntacticPrimaryKind::RepeatedSequence(
                self.delimited_definitions_list(TokenKind::RBrace)?,
            ),
            TokenKind::LParen => SyntacticPrimaryKind::GroupedSequence(
                self.delimited_definitions_list(TokenKind::RParen)?,
            ),
            TokenKind::Identifier(name) => {
                self.next()?;
                SyntacticPrimaryKind::MetaIdentifier(name)
            }
            TokenKind::Terminal(text) => {
                self.next()?;
                SyntacticPrimaryKind::TerminalString(text)
            }
            TokenKind::SpecialSeq(text) => {
                self.next()?;
                SyntacticPrimaryKind::SpecialSequence(text)
            }
            _ => SyntacticPrimaryKind::EmptySequence,
        };

        Ok(SyntacticPrimary {
            span: Span::new(start, self.prev_span.end),
            kind,
        })
    }

    fn delimited_definitions_list(
        &mut self,
        right_delimiter: TokenKind,
    ) -> Result<Vec<SingleDefinition<'src>>, SyntaxError> {
        self.next()?;
        let definitions = self.definitions_list()?;
        self.expect(right_delimiter)?;

        Ok(definitions)
    }
}
