mod configuration;
mod formatter;

pub use configuration::Configuration;
pub use formatter::Formatter;

#[cfg(test)]
mod tests {
    use ebnf_parser::{Lexer, Parser};

    use crate::configuration::Configuration;

    use super::*;

    #[test]
    fn format() {
        let input = include_str!("../grammar.ebnf");
        let out = Formatter::new(
            Parser::new(Lexer::new(input)).parse().unwrap(),
            input,
            &Configuration::default(),
        )
        .format();
        println!("{out}")
    }
}
