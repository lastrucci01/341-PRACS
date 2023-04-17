#[derive(Debug, Clone)]
pub enum LexType {
    End,
    Comment,
    ShortString,
    Num,
    Alpha,
    Punct,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    End,
    Keyword,
    Identifier,
    AssignmentOperator,
    LogicalOperator,
    ComparisionOperator,
    NumberLiteral,
    StringLiteral,
    Punctuation,
    Comment,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: (usize, usize),
}

use lazy_static::lazy_static;

lazy_static! {
    static ref LOGIC_OPS: Vec<String> =
        vec![String::from("^"), String::from("v"), String::from("!"),];
    static ref CMPR_OPS: Vec<String> =
        vec![String::from("E"), String::from("<"), String::from(">"),];
}

impl Token {
    pub fn new(lexeme: String, lex_type: LexType, position: (usize, usize)) -> Option<Self> {
        match lex_type {
            LexType::Comment => {
                for c in lexeme.chars() {
                    if !c.is_ascii() {
                        panic!("\x1B[31m{}\x1B[0m", "LEX ERROR - NON ASCII IN COMMENT")
                    }
                }

                if lexeme.len() != 17 {
                    panic!("\x1B[31m{}\x1B[0m", "LEX ERROR - COMMENT LENGTH ERROR")
                }

                Some(Self {
                    token_type: TokenType::Comment,
                    lexeme,
                    position,
                })
            }
            LexType::ShortString => {
                for c in lexeme.chars() {
                    if !c.is_ascii() {
                        panic!("\x1B[31m{}\x1B[0m", "LEX ERROR - NON ASCII IN STRING")
                    }
                }
                if lexeme.len() != 17 {
                    panic!("\x1B[31m{}\x1B[0m", "LEX ERROR - STRING LENGTH ERROR")
                }
                Some(Self {
                    token_type: TokenType::StringLiteral,
                    lexeme,
                    position,
                })
            }
            LexType::Alpha => {
                let lex_len = lexeme.clone().len();
                if lex_len == 1 {
                    return Some(Token {
                        token_type: TokenType::Keyword,
                        lexeme,
                        position,
                    });
                }

                let contains_num = lexeme.chars().any(|c| c.is_numeric());
                if contains_num {
                    return Some(Token {
                        token_type: TokenType::Identifier,
                        lexeme,
                        position,
                    });
                }

                println!("{} - {:?}", lexeme, lex_type);
                None
            }
            LexType::Punct => match lexeme.as_str() {
                ":=" => Some(Self {
                    token_type: TokenType::AssignmentOperator,
                    lexeme,
                    position,
                }),
                _ => {
                    if LOGIC_OPS.contains(&lexeme) {
                        Some(Self {
                            token_type: TokenType::LogicalOperator,
                            lexeme,
                            position,
                        })
                    } else if CMPR_OPS.contains(&lexeme) {
                        Some(Self {
                            token_type: TokenType::ComparisionOperator,
                            lexeme,
                            position,
                        })
                    } else {
                        Some(Self {
                            token_type: TokenType::Punctuation,
                            lexeme,
                            position,
                        })
                    }
                }
            },
            LexType::Num => Some(Self {
                token_type: TokenType::NumberLiteral,
                lexeme: lexeme,
                position,
            }),
            LexType::End => Some(Self {
                token_type: TokenType::End,
                lexeme,
                position,
            }),
        }
    }
}
