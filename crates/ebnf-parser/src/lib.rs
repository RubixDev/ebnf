pub mod ast;
pub mod error;
mod lexer;
mod parser;
pub mod span;
mod token;

pub use lexer::Lexer;
pub use parser::*;
pub use token::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex() {
        // let mut lexer = lexer::Lexer::new(include_str!("../grammar.ebnf"));
        let mut lexer = Lexer::new("'a' , \"ba\" | () {} [] ; ? asdasd ? (* as (d *) asd");
        while let Ok(Some(token)) = lexer.next_token() {
            println!("{token:?}");
        }
    }

    #[test]
    fn parse() {
        let text = include_str!("../grammar.ebnf");
        let res = Parser::new(Lexer::new(text)).parse();
        match res {
            Ok(res) => {
                println!("{:#?}", res.syntax);
                for (k, v) in res.comments {
                    println!(
                        "{}: {:?} -- {:?}",
                        k,
                        v.iter().map(|c| c.text).collect::<Vec<_>>(),
                        res.tokens.get(k),
                    );
                }
            }
            Err(err) => eprintln!(
                "\x1b[31m{}\x1b[1m{}\x1b[22m{}\x1b[0m\n{}",
                &text[(err.span.start - 20)..(err.span.start)],
                &text[err.span.start..err.span.end],
                &text[(err.span.end)..(err.span.end + 20)],
                err.message,
            ),
        }
    }
}
