use crate::lexer::{Token, TokenKind};

struct Executor {}

impl Executor {
    fn calculate(postfix: Vec<Token>) -> i32 {
        let mut stack: Vec<Token> = vec![];

        for it in postfix {
            if it.kind == TokenKind::Number {
                stack.push(it)
            } else if it.kind == TokenKind::Operator {
                let b = stack
                    .pop()
                    .expect("operator need at least 2 operand")
                    .token
                    .parse::<i32>()
                    .unwrap();
                let a = stack
                    .pop()
                    .expect("operator need at least 2 operand")
                    .token
                    .parse::<i32>()
                    .unwrap();

                let v = match &it.token[..] {
                    "*" => a * b,
                    "/" => a / b,
                    "+" => a + b,
                    "-" => a - b,
                    _ => 0,
                };

                stack.push(Token {
                    token: v.to_string(),
                    kind: TokenKind::Number,
                });
            }
        }

        stack.pop().unwrap().token.parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn wrong_datatype() {
        const FORMULA: &str = "3 + 5 / 8 * 3 + 2";
        let lexer = Lexer::parse_string(FORMULA);

        let parsed = Parser::parse(lexer.tokens);
        println!("parsed {:?}", parsed);

        let executed = Executor::calculate(parsed);
        assert_eq!(executed, 5)
    }

    #[test]
    fn with_multiply() {
        const FORMULA: &str = "3 + 5 * 2"; // 3 5 2 * + // 13
        let lexer = Lexer::parse_string(FORMULA);

        let parsed = Parser::parse(lexer.tokens);
        println!("parsed {:?}", parsed);

        let executed = Executor::calculate(parsed);
        assert_eq!(executed, 13);
    }

    #[test]
    fn basic_oneliner_math() {
        const FORMULA: &str = "3 + 5"; // 8
        let lexer = Lexer::parse_string(FORMULA);
        let parsed = Parser::parse(lexer.tokens);
        let executed = Executor::calculate(parsed);
        assert_eq!(executed, 8);
    }
}
