use std::vec;

#[cfg(debug_assertions)]
use ebnf_parser::Token;
use ebnf_parser::{ast::*, CommentMap, ParseResult, TokenKind};

use crate::configuration::{Configuration, NewlineKind, QuoteStyle};

enum Special {
    /// A newline according to the current config
    Newline,
    /// The current indent as spaces
    Indent,
    /// A Newline followed by an Indent
    NewlineIndent,
    /// The current indent as spaces minus the given length
    RestIndent(usize),
    /// A MergingSpace or NewlineIndent depending on the current line length
    SpaceOrNewline,
    /// A space when the previous character is not a space
    MergingSpace,
}

enum PushKind<'a> {
    Char(char),
    Str(&'a str),
    Special(Special),
}

impl From<char> for PushKind<'_> {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl<'a> From<&'a str> for PushKind<'a> {
    fn from(s: &'a str) -> Self {
        Self::Str(s)
    }
}

impl From<Special> for PushKind<'_> {
    fn from(s: Special) -> Self {
        Self::Special(s)
    }
}

pub struct Formatter<'src, 'config> {
    syntax: Option<Syntax<'src>>,
    text: &'src str,
    config: &'config Configuration,
    indent: usize,
    output: String,
    curr_line_len: usize,
    #[cfg(debug_assertions)]
    tokens: vec::IntoIter<Token<'src>>,
    #[cfg(debug_assertions)]
    curr_tok: Option<Token<'src>>,
    tok_index: usize,
    comments: CommentMap<'src>,
    /// Is true while ignoring formatting for a rule to prevent pushing to `output` while still
    /// progressing `tokens`.
    no_push: bool,
}

impl<'src, 'config> Formatter<'src, 'config> {
    pub fn new(
        parse_result: ParseResult<'src>,
        text: &'src str,
        config: &'config Configuration,
    ) -> Self {
        Self {
            syntax: Some(parse_result.syntax),
            text,
            config,
            indent: 0,
            output: String::new(),
            curr_line_len: 0,
            #[cfg(debug_assertions)]
            tokens: parse_result.tokens.into_iter(),
            #[cfg(debug_assertions)]
            curr_tok: None,
            tok_index: usize::MAX,
            comments: parse_result.comments,
            no_push: false,
        }
    }

    pub fn format(mut self) -> String {
        self.next_tok();
        let syntax = self
            .syntax
            .take()
            .expect("set to Some(..) in Formatter::new and this method is only called once");
        self.format_syntax(syntax);
        self.output
    }

    fn next_tok(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.curr_tok = self.tokens.next();
        }
        self.tok_index = self.tok_index.wrapping_add(1);
    }

    fn push(&mut self, kind: PushKind) {
        match kind {
            PushKind::Char(c) => self.push_char(c),
            PushKind::Str(s) => self.push_str(s),
            PushKind::Special(s) => self.push_special(s),
        }
    }

    fn push_char(&mut self, char: char) {
        if self.no_push {
            return;
        }
        self.curr_line_len += 1;
        self.output.push(char);
    }

    fn push_str(&mut self, text: &str) {
        if self.no_push {
            return;
        }
        self.curr_line_len += text.chars().count();
        self.output.push_str(text);
    }

    fn push_special(&mut self, special: Special) {
        if self.no_push {
            return;
        }
        match special {
            Special::Newline => {
                // Trim trailing spaces
                self.output
                    .truncate(self.output.trim_end_matches(' ').len());

                match self.config.newline_kind {
                    NewlineKind::Unix => self.output.push('\n'),
                    NewlineKind::Windows => self.output.push_str("\r\n"),
                };
                self.curr_line_len = 0;
            }
            Special::Indent => self.push_str(&" ".repeat(self.indent)),
            Special::NewlineIndent => {
                self.push_special(Special::Newline);
                self.push_special(Special::Indent);
            }
            Special::RestIndent(len) => self.push_str(&" ".repeat(self.indent - len)),
            Special::SpaceOrNewline => {
                if self.curr_line_len >= self.config.line_width {
                    self.push_special(Special::NewlineIndent);
                } else {
                    self.push_special(Special::MergingSpace);
                }
            }
            Special::MergingSpace => {
                if !self.output.ends_with(' ') {
                    self.push_char(' ');
                }
            }
        }
    }

    fn push_token(&mut self, token: TokenKind, prefix: Option<PushKind>, suffix: Option<PushKind>) {
        self.check_comments();
        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                token,
                self.curr_tok
                    .as_ref()
                    .unwrap_or_else(|| panic!("expected TokenKind {:?} but was None", token))
                    .kind
            );
        }
        self.next_tok();

        if let Some(prefix) = prefix {
            self.push(prefix);
        }
        match token {
            TokenKind::Terminal(text) => {
                let quote = match self.config.quote_style {
                    QuoteStyle::Single if text.contains('\'') => '"',
                    QuoteStyle::Single => '\'',
                    QuoteStyle::Double if text.contains('"') => '\'',
                    QuoteStyle::Double => '"',
                };
                self.push_char(quote);
                self.push_str(text);
                self.push_char(quote);
            }
            _ => self.push_str(&token.to_string()),
        }
        if let Some(suffix) = suffix {
            self.push(suffix);
        }
    }

    fn check_comments(&mut self) {
        if let Some(comments) = self.comments.remove(&self.tok_index) {
            let mut prev_comment: Option<Comment> = None;
            for comment in comments {
                // Insert blank line when there was one before
                if let Some(prev_comment) = prev_comment {
                    let text_between = &self.text[prev_comment.span.end..comment.span.start];
                    if text_between.contains("\n\n") || text_between.contains("\r\n\r\n") {
                        self.push_special(Special::Newline);
                    }
                }

                self.format_comment(comment.text);
                prev_comment = Some(comment);
            }
        }
    }

    fn format_syntax(&mut self, node: Syntax) {
        let mut blocks: Vec<Vec<SyntaxRule>> = vec![vec![]];
        for node in node.rules {
            if let Some(prev_node) = blocks
                .last()
                .expect("Vector initialized with one element and never remove any element")
                .last()
            {
                let text_between = &self.text[prev_node.span.end..node.span.start];
                if text_between.contains("\n\n") || text_between.contains("\r\n\r\n") {
                    blocks.push(vec![]);
                }
            }
            blocks
                .last_mut()
                .expect("Vector initialized with one element and never remove any element")
                .push(node);
        }
        for block in blocks {
            self.format_rule_block(block);
            self.push_special(Special::Newline);
        }
        self.check_comments();
    }

    fn format_rule_block(&mut self, block: Vec<SyntaxRule>) {
        self.indent = block
            .iter()
            .map(|rule| rule.name.len())
            .max()
            .expect("Every block consists of at least one rule")
            + 1;
        for rule in block {
            self.format_syntax_rule(rule);
        }
    }

    fn format_syntax_rule(&mut self, node: SyntaxRule) {
        // Check for ignore comment
        if let Some(comments) = self.comments.get(&self.tok_index) {
            if comments
                .iter()
                .any(|comment| comment.text.contains(&self.config.ignore_rule_comment_text))
            {
                self.check_comments();
                let raw_text = &self.text[node.span.start..node.span.end];
                for line in raw_text.split('\n') {
                    self.push_str(line.trim_end_matches('\r'));
                    self.push_special(Special::Newline);
                }
                self.no_push = true;
            }
        }

        // Format
        self.push_token(TokenKind::Identifier(node.name), None, None);
        self.push_special(Special::RestIndent(node.name.len()));
        self.push_token(TokenKind::Equal, None, Some(' '.into()));
        self.format_definitions_list(node.definitions);
        self.push_token(
            TokenKind::Semicolon,
            Some(Special::MergingSpace.into()),
            None,
        );
        self.push_special(Special::Newline);

        self.no_push = false;
    }

    fn format_definitions_list(&mut self, node: Vec<SingleDefinition>) {
        // Leave inline when every definition has at most `max_child_len_for_inline_definition_list` terms
        // AND either len <= `max_count_for_inline_definition_list`
        //     or at least `min_terminals_percent_for_inline_definition_list`% of the definitions
        //     are a single TerminalString
        let inline = node
            .iter()
            .all(|node| node.terms.len() <= self.config.max_child_len_for_inline_definition_list)
            && (node.len() <= self.config.max_count_for_inline_definition_list
                || node
                    .iter()
                    .filter(|node| {
                        matches!(
                            node.terms.as_slice(),
                            [SyntacticTerm {
                                factor: SyntacticFactor {
                                    primary: SyntacticPrimary {
                                        kind: SyntacticPrimaryKind::TerminalString(..),
                                        ..
                                    },
                                    repetition: None,
                                    ..
                                },
                                exception: None,
                                ..
                            }]
                        )
                    })
                    .count() as f64
                    >= (node.len() as f64)
                        * self
                            .config
                            .min_terminals_percent_for_inline_definition_list
                            .get_f64());

        let last = node.len().saturating_sub(1);
        for (index, node) in node.into_iter().enumerate() {
            self.format_single_definition(node);
            if index != last {
                self.push_token(
                    TokenKind::Pipe,
                    Some(match inline {
                        true => Special::SpaceOrNewline.into(),
                        false => Special::NewlineIndent.into(),
                    }),
                    Some(' '.into()),
                );
            }
        }
    }

    fn format_single_definition(&mut self, node: SingleDefinition) {
        let last = node.terms.len().saturating_sub(1);
        for (index, node) in node.terms.into_iter().enumerate() {
            self.format_syntactic_term(node);
            if index != last {
                self.push_token(
                    TokenKind::Comma,
                    Some(Special::SpaceOrNewline.into()),
                    Some(' '.into()),
                );
            }
        }
    }

    fn format_syntactic_term(&mut self, node: SyntacticTerm) {
        let (prefix, suffix) = match (&node.factor, &node.exception) {
            (
                SyntacticFactor {
                    primary:
                        SyntacticPrimary {
                            kind: SyntacticPrimaryKind::RepeatedSequence(_),
                            ..
                        },
                    ..
                },
                Some(SyntacticFactor {
                    primary:
                        SyntacticPrimary {
                            kind: SyntacticPrimaryKind::EmptySequence,
                            ..
                        },
                    ..
                }),
            ) => (None, None),
            (
                _,
                Some(SyntacticFactor {
                    primary:
                        SyntacticPrimary {
                            kind: SyntacticPrimaryKind::EmptySequence,
                            ..
                        },
                    ..
                }),
            ) => (Some(Special::MergingSpace.into()), None),
            _ => (Some(Special::MergingSpace.into()), Some(' '.into())),
        };

        self.format_syntactic_factor(node.factor);
        if let Some(exception) = node.exception {
            self.push_token(TokenKind::Dash, prefix, suffix);
            self.format_syntactic_factor(exception);
        }
    }

    fn format_syntactic_factor(&mut self, node: SyntacticFactor) {
        if let Some(repetition) = node.repetition {
            self.push_token(TokenKind::Integer(repetition), None, None);
            self.push_token(
                TokenKind::Star,
                Some(Special::MergingSpace.into()),
                Some(' '.into()),
            );
        }
        self.format_syntactic_primary(node.primary);
    }

    fn format_syntactic_primary(&mut self, node: SyntacticPrimary) {
        match node.kind {
            SyntacticPrimaryKind::OptionalSequence(node) => self.format_delimited_definitions_list(
                node,
                TokenKind::LBracket,
                TokenKind::RBracket,
            ),
            SyntacticPrimaryKind::RepeatedSequence(node) => {
                self.format_delimited_definitions_list(node, TokenKind::LBrace, TokenKind::RBrace)
            }
            SyntacticPrimaryKind::GroupedSequence(node) => {
                self.format_delimited_definitions_list(node, TokenKind::LParen, TokenKind::RParen)
            }
            SyntacticPrimaryKind::MetaIdentifier(name) => {
                self.push_token(TokenKind::Identifier(name), None, None)
            }
            SyntacticPrimaryKind::TerminalString(text) => {
                self.push_token(TokenKind::Terminal(text), None, None)
            }
            SyntacticPrimaryKind::SpecialSequence(text) => {
                self.push_token(TokenKind::SpecialSeq(text), None, None)
            }
            SyntacticPrimaryKind::EmptySequence => {}
        }
    }

    fn format_delimited_definitions_list(
        &mut self,
        node: Vec<SingleDefinition>,
        open: TokenKind,
        close: TokenKind,
    ) {
        let saved_indent = self.indent;
        self.indent = self.curr_line_len;
        self.push_token(open, None, Some(' '.into()));
        self.format_definitions_list(node);
        self.push_token(close, Some(Special::MergingSpace.into()), None);
        self.indent = saved_indent;
    }

    fn format_comment(&mut self, mut text: &str) {
        if self.curr_line_len != 0 {
            self.push_special(Special::MergingSpace);
            self.push_str("(* ");
            self.push_str(text.trim());
            self.push_str(" *) ");
        } else if text.contains('\n') {
            let saved_indent = self.indent;
            self.indent = self.config.mutliline_comment_indent;

            self.push_str("(*");
            self.push_special(Special::Newline);

            let current_comment_indent = text
                .trim_start_matches(|c| c == '\n' || c == '\r')
                .chars()
                .take_while(|c| *c == ' ')
                .count();
            text = text.trim();

            for mut line in text.split('\n') {
                line = line.trim_end_matches('\r');
                if !line.trim().is_empty() {
                    self.push_special(Special::Indent);
                }

                // Trim any existing indent up to `current_comment_indent`
                let mut line_start = 0;
                while line_start < current_comment_indent
                    && line.as_bytes().get(line_start) == Some(&b' ')
                {
                    line_start += 1;
                }

                self.push_str(&line[line_start..]);
                self.push_special(Special::Newline);
            }

            self.push_str("*)");
            self.indent = saved_indent;
            self.push_special(Special::Newline);
        } else {
            self.push_str("(* ");
            self.push_str(text.trim());
            self.push_str(" *)");
            self.push_special(Special::Newline);
        }
    }
}
