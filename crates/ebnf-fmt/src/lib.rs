pub mod configuration;
mod formatter;

pub use configuration::Configuration;
use ebnf_parser::{error::SyntaxError, Lexer, Parser};
pub use formatter::Formatter;

pub fn format_code(text: &str, config: &Configuration) -> Result<String, SyntaxError> {
    Ok(Formatter::new(
        Parser::new(Lexer::new(text)).parse()?,
        text,
        config,
        |text| text,
    )
    .format())
}

pub fn format_code_with_comment_formatter(
    text: &str,
    config: &Configuration,
    comment_formatter: impl FnMut(String) -> String,
) -> Result<String, SyntaxError> {
    Ok(Formatter::new(
        Parser::new(Lexer::new(text)).parse()?,
        text,
        config,
        comment_formatter,
    )
    .format())
}

#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;

    use super::*;

    #[test]
    fn format() {
        let input = include_str!("../../ebnf-parser/grammar.ebnf");
        let output = format_code(input, &Configuration::default()).unwrap();
        println!("{output}");
        assert!(!output.ends_with("\n\n"));
    }
}
