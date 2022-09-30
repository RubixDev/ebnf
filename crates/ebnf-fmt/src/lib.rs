mod configuration;
mod formatter;

pub use configuration::Configuration;
pub use formatter::Formatter;

#[cfg(test)]
mod tests {
    use ebnf_parser::{Lexer, Parser};

    use crate::configuration::{Configuration, NewlineKind, QuoteStyle};

    use super::*;

    #[test]
    fn format() {
        let input = include_str!("../grammar.ebnf");
        let out = Formatter::new(
            Parser::new(Lexer::new(input)).parse().unwrap(),
            input,
            &Configuration {
                line_width: 100,
                newline_kind: NewlineKind::Unix,
                ignore_rule_comment_text: "ebnf-fmt ignore".to_string(),
                quote_style: QuoteStyle::Single,
                mutliline_comment_indent: 2,
            },
        )
        .format();
        println!("{out}")
    }
}
