use std::vec;

const OPERATOR: &str = "*/+-()";
const WHITESPACE: &str = " \n";

#[derive(Debug)]
pub enum TokenKind {
    Number,
    Operator,
    Word,
    Whitespace,
}

#[derive(Debug)]
pub struct Token {
    token: String,
    kind: TokenKind,
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}
impl Lexer {
    fn parse_string(text: &str) -> Lexer {
        let mut _token: Vec<Token> = vec![];
        fn add_buff_parsed(__token: &mut Vec<Token>, __buff: &String) {
            __token.push(match __buff.trim().parse::<i32>() {
                Ok(_) => Token {
                    token: __buff.clone(),
                    kind: TokenKind::Number,
                },
                Err(_) => Token {
                    token: __buff.clone(),
                    kind: if WHITESPACE.contains(__buff) {
                        TokenKind::Whitespace
                    } else {
                        TokenKind::Word
                    },
                },
            });
        }

        let mut buff = String::from("");

        for it in text.chars() {
            if OPERATOR.contains(it) {
                if !buff.is_empty() {
                    add_buff_parsed(&mut _token, &buff);
                    buff.clear();
                }

                _token.push(Token {
                    token: it.to_string(),
                    kind: TokenKind::Operator,
                });
            } else {
                buff.push(it);
            }
        }

        if !buff.is_empty() {
            add_buff_parsed(&mut _token, &buff);
            buff.clear();
        }

        Lexer { tokens: _token }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn using_bracket() {
        const FORMULA: &str = "3 + (5 / 8) * 3 + 2";
        let lexer = Lexer::parse_string(FORMULA);
        println!("{:?}", lexer)
    }

    #[test]
    fn long_oneliner_math() {
        const FORMULA: &str = "3 + 5 / 8 * 3 + 2";
        let lexer = Lexer::parse_string(FORMULA);
        println!("{:?}", lexer)
    }

    #[test]
    fn basic_oneliner_math() {
        const FORMULA: &str = "3 + 5";
        let lexer = Lexer::parse_string(FORMULA);
        println!("{:?}", lexer)
    }
}
