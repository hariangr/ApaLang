use super::lexer::Token;
use crate::lexer::TokenKind;
use std::vec;

struct Precedence(&'static str, i32);

const PRECEDENCE: [Precedence; 5] = [
    Precedence { 0: "+", 1: 2 },
    Precedence { 0: "-", 1: 2 },
    Precedence { 0: "*", 1: 3 },
    Precedence { 0: "/", 1: 3 },
    Precedence { 0: "^", 1: 4 },
];
impl Precedence {
    fn priority_by_symbol(symbol: &str) -> Option<i32> {
        for it in PRECEDENCE {
            if it.0 == symbol {
                return Some(it.1);
            }
        }

        None
    }
}
impl Token {
    fn get_precedence(&self) -> i32 {
        Precedence::priority_by_symbol(&self.token).expect("token doesn't have precedence")
    }
}

struct Parser {}
impl Parser {
    /// Using shunting algorithm
    fn parse(tokens: Vec<Token>) -> Vec<Token> {
        let mut output: Vec<Token> = vec![];
        let mut operator: Vec<Token> = vec![];

        for it in tokens {
            if it.kind == TokenKind::Number {
                output.push(it);
            } else if it.kind == TokenKind::Operator {
                // let

                if operator.len() == 0 {
                    operator.push(it);
                } else {
                    let prior_token = operator
                        .pop()
                        .expect("operator should have at least one before comparison occur");

                    if it.get_precedence() < prior_token.get_precedence()
                        || it.get_precedence() == prior_token.get_precedence()
                    {
                        println!("A");
                        operator.push(it);
                        output.push(prior_token);
                    } else {
                        println!("B");
                        operator.push(prior_token);
                        operator.push(it);
                    }
                }
            }
        }

        for it in operator {
            output.push(it);
        }

        println!("OUTPUT {:?}", output);
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn long_math() {
        let formula = "5 + 3 / 3 + 10"; // 5 3 3 / 10 + +

        let parsed = Lexer::parse_string(formula).tokens;

        let r = Parser::parse(parsed);
    }

    #[test]
    fn simple_math() {
        let formula = "5 + 3";
        let parsed = Lexer::parse_string(formula).tokens;

        let r = Parser::parse(parsed);

        println!("Entah {:?}", r);
    }
}
