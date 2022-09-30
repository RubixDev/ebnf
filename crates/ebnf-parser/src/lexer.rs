use std::{mem, str::Chars};

use crate::{
    error::SyntaxError,
    span::Span,
    token::{Token, TokenKind},
};

macro_rules! simple_token {
    ($self:ident, $token:expr) => {{
        let start = $self.index;
        $self.next();
        Ok(Token::new($token, Span::new(start, $self.index)))
    }};
}

pub struct Lexer<'src> {
    text: &'src str,
    src: Chars<'src>,
    curr_char: Option<char>,
    next_char: Option<char>,
    pub(crate) index: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(text: &'src str) -> Self {
        let mut lexer = Lexer {
            text,
            src: text.chars(),
            curr_char: None,
            next_char: None,
            index: 0,
        };
        lexer.next();
        lexer.next();
        lexer
    }

    fn next(&mut self) {
        if let Some(curr_char) = self.curr_char {
            self.index += curr_char.len_utf8();
        }
        mem::swap(&mut self.curr_char, &mut self.next_char);
        self.next_char = self.src.next();
    }

    pub fn next_token(&mut self) -> Result<Option<Token<'src>>, SyntaxError> {
        while let Some(' ' | '\n' | '\t' | '\r') = self.curr_char {
            self.next();
        }
        if let Some(curr_char) = self.curr_char {
            let token_result = match curr_char {
                '{' => simple_token!(self, TokenKind::LBrace),
                '}' => simple_token!(self, TokenKind::RBrace),
                '[' => simple_token!(self, TokenKind::LBracket),
                ']' => simple_token!(self, TokenKind::RBracket),
                '(' if self.next_char != Some('*') => simple_token!(self, TokenKind::LParen),
                ')' => simple_token!(self, TokenKind::RParen),
                '|' => simple_token!(self, TokenKind::Pipe),
                ',' => simple_token!(self, TokenKind::Comma),
                ';' => simple_token!(self, TokenKind::Semicolon),
                '=' => simple_token!(self, TokenKind::Equal),
                '*' => simple_token!(self, TokenKind::Star),
                '-' => simple_token!(self, TokenKind::Dash),
                '(' => self.parse_comment(),
                '\'' | '"' => self.parse_terminal(),
                '?' => self.parse_special_seq(),
                c if c.is_ascii_alphabetic() => self.parse_identifier(),
                c if c.is_ascii_digit() => self.parse_integer(),
                c => {
                    let span_start = self.index;
                    self.next();
                    Err(SyntaxError::new(
                        Span::new(span_start, self.index),
                        format!("Illegal character '{}'", c).into(),
                    ))
                }
            };
            match token_result {
                Ok(token) => Ok(Some(token)),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }

    fn delimeted_str(&mut self, delimeter: Option<char>) -> &'src str {
        self.next(); // opening delimeter
        let content_start = self.index;
        while self.curr_char.is_some() && self.curr_char != delimeter {
            self.next();
        }
        let content_end = self.index;
        self.next(); // closing delimeter
        &self.text[content_start..content_end]
    }

    fn parse_comment(&mut self) -> Result<Token<'src>, SyntaxError> {
        debug_assert!(
            self.curr_char == Some('(') && self.next_char == Some('*'),
            "Expected '(' and '*', was {:?} and {:?}",
            self.curr_char,
            self.next_char,
        );

        let span_start = self.index;

        self.next();
        self.next();
        let content_start = self.index;
        while self.curr_char.is_some()
            && !(self.curr_char == Some('*') && self.next_char == Some(')'))
        {
            self.next();
        }
        let content_end = self.index;
        self.next();
        self.next();

        let content = &self.text[content_start..content_end];

        Ok(Token::new(
            TokenKind::Comment(content),
            Span::new(span_start, self.index),
        ))
    }

    fn parse_terminal(&mut self) -> Result<Token<'src>, SyntaxError> {
        debug_assert!(
            self.curr_char == Some('\'') || self.curr_char == Some('"'),
            "Expected quote, was {:?}",
            self.curr_char,
        );

        let quote = self.curr_char;
        let span_start = self.index;
        let content = self.delimeted_str(quote).trim();

        Ok(Token::new(
            TokenKind::Terminal(content),
            Span::new(span_start, self.index),
        ))
    }

    fn parse_special_seq(&mut self) -> Result<Token<'src>, SyntaxError> {
        debug_assert!(
            self.curr_char == Some('?'),
            "Expected '?', was {:?}",
            self.curr_char,
        );

        let span_start = self.index;
        let content = self.delimeted_str(Some('?')).trim();

        Ok(Token::new(
            TokenKind::SpecialSeq(content),
            Span::new(span_start, self.index),
        ))
    }

    fn parse_identifier(&mut self) -> Result<Token<'src>, SyntaxError> {
        debug_assert!(
            self.curr_char.map_or(false, |c| c.is_ascii_alphabetic()),
            "Expected letter, was {:?}",
            self.curr_char,
        );

        let span_start = self.index;
        let content_start = self.index;
        self.next(); // first letter
        while self
            .curr_char
            .map_or(false, |c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.next();
        }
        let content_end = self.index;
        let content = &self.text[content_start..content_end];

        Ok(Token::new(
            TokenKind::Identifier(content),
            Span::new(span_start, self.index),
        ))
    }

    fn parse_integer(&mut self) -> Result<Token<'src>, SyntaxError> {
        debug_assert!(
            self.curr_char.map_or(false, |c| c.is_ascii_digit()),
            "Expected digit, was {:?}",
            self.curr_char,
        );

        let span_start = self.index;
        let content_start = self.index;
        self.next(); // first digit
        while self.curr_char.map_or(false, |c| c.is_ascii_digit()) {
            self.next();
        }
        let content_end = self.index;
        let slice = &self.text[content_start..content_end];
        let num = match slice.parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(SyntaxError::new(
                    Span::new(span_start, self.index),
                    "Number does not fit into `usize` type".into(),
                ))
            }
        };

        Ok(Token::new(
            TokenKind::Integer(num),
            Span::new(span_start, self.index),
        ))
    }
}
