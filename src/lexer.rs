use std::vec;

const OPERATOR: &str = "*/+-()";
const WHITESPACE: &str = " \n";

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number,
    Operator,
    Word,
    Whitespace,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,
    pub kind: TokenKind,
}

pub trait SimplifyTokens {
    fn simplify_tokens(&self) -> String;
    fn no_whitespace(&self) -> Vec<Token>;
}
impl SimplifyTokens for Vec<Token> {
    fn simplify_tokens(&self) -> String {
        let mut r = String::from("");

        for it in self {
            r += &it.token;
        }

        r
    }

    fn no_whitespace(&self) -> Vec<Token> {
        let mut res = vec![];
        for it in self {
            if it.kind != TokenKind::Whitespace {
                res.push(it.clone())
            }
        }

        res
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
}
impl Lexer {
    pub fn parse_string(text: &str) -> Lexer {
        let mut _token: Vec<Token> = vec![];

        fn __add_buff_parsed(__token: &mut Vec<Token>, __buff: &String) {
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
                    __add_buff_parsed(&mut _token, &buff);
                    buff.clear();
                }

                _token.push(Token {
                    token: it.to_string(),
                    kind: TokenKind::Operator,
                });
            } else if WHITESPACE.contains(it) {
                if !buff.is_empty() {
                    __add_buff_parsed(&mut _token, &buff);
                    buff.clear();
                }

                _token.push(Token {
                    token: it.to_string(),
                    kind: TokenKind::Whitespace,
                });
            } else {
                buff.push(it);
            }
        }

        if !buff.is_empty() {
            __add_buff_parsed(&mut _token, &buff);
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

        let res = lexer.tokens.simplify_tokens();
        assert_eq!(res, FORMULA);

        let res = lexer.tokens.no_whitespace().simplify_tokens();
        assert_eq!(res, "3+(5/8)*3+2");
    }

    #[test]
    fn long_oneliner_math() {
        const FORMULA: &str = "3 + 5 / 8 * 3 + 2";
        let lexer = Lexer::parse_string(FORMULA);

        let res = lexer.tokens.simplify_tokens();
        assert_eq!(res, FORMULA);

        let res = lexer.tokens.no_whitespace().simplify_tokens();
        assert_eq!(res, "3+5/8*3+2");
    }

    #[test]
    fn basic_oneliner_math() {
        const FORMULA: &str = "3 + 5";
        let lexer = Lexer::parse_string(FORMULA);

        let res = lexer.tokens.simplify_tokens();
        assert_eq!(res, FORMULA);

        let res = lexer.tokens.no_whitespace().simplify_tokens();
        assert_eq!(res, "3+5");
    }
}
