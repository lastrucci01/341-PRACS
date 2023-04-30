use crate::token::{LexType, Token};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line_offset: (usize, usize),
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line_offset: (0, 0),
        }
    }

    pub fn consume_enclosed(&mut self, is_comment: bool) -> String {
        let start = self.position;
        self.position += 1;
        match is_comment {
            true => {
                while self.position < self.source.len() && self.source[self.position] != '*' {
                    self.position += 1;
                }
            }
            false => {
                while self.position < self.source.len() && self.source[self.position] != '"' {
                    self.position += 1;
                }
            }
        }
        self.position += 1;
        String::from_iter(&self.source[start..self.position])
    }

    pub fn consume_numeric(&mut self) -> String {
        let start = self.position;
        if self.source[self.position] == '-' {
            self.position += 1;
        }
        while self.position < self.source.len()
            && (self.source[self.position].is_numeric() 
            || self.source[self.position] == '.') {
            self.position += 1;
        }
        String::from_iter(&self.source[start..self.position])
    }

    pub fn consume_alphanumeric(&mut self) -> String {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position].is_alphanumeric() {
            self.position += 1;
        }
        String::from_iter(&self.source[start..self.position])
    }

    pub fn consume_punctuation(&mut self) -> String {
        let start = self.position;
        match self.source[start] {
            ':' => {
                self.position += 2;
                String::from_iter(&self.source[start..self.position])
            }
            _ => {
                self.position += 1;
                String::from_iter(&self.source[start..self.position])
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.source.len() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }

        if self.position == self.source.len() {
            self.position += 1;
            return Token::new(
                String::from("$"),
                LexType::End,
                (self.line_offset.0, self.position - self.line_offset.1),
            );
        }

        if self.position >= self.source.len() {
            return None;
        }

        if self.source[self.position] == '"' || self.source[self.position] == '*' {
            return match self.source[self.position] {
                '"' => {
                    let lexeme = self.consume_enclosed(false);
                    // println!("stringy lex - {} // pos - {}", lexeme, self.position);
                    Token::new(
                        lexeme,
                        LexType::ShortString,
                        (self.line_offset.0, self.position - self.line_offset.1),
                    )
                }
                '*' => {
                    let lexeme = self.consume_enclosed(true);
                    // println!("commenty lex - {} // pos - {}", lexeme, self.position);
                    Token::new(
                        lexeme,
                        LexType::Comment,
                        (self.line_offset.0, self.position - self.line_offset.1),
                    )
                }
                _ => panic!("how did we get here? ('shortstring or comment' match)"),
            };
        }

        if self.source[self.position].is_numeric() || self.source[self.position] == '-' {
            let lexeme = self.consume_numeric();
            // println!("numy lex - {} // pos - {}", lexeme, self.position);
            return Token::new(
                lexeme,
                LexType::Num,
                (self.line_offset.0, self.position - self.line_offset.1),
            );
        }

        if self.source[self.position].is_alphanumeric() {
            let lexeme = self.consume_alphanumeric();
            // println!("alpha lex - {} // pos - {}", lexeme, self.position);
            return Token::new(
                lexeme,
                LexType::Alpha,
                (self.line_offset.0, self.position - self.line_offset.1),
            );
        }

        if self.source[self.position].is_ascii_punctuation() {
            let lexeme = self.consume_punctuation();
            // println!("puncty lex - {} // pos - {}", lexeme, self.position);
            return Token::new(
                lexeme,
                LexType::Punct,
                (self.line_offset.0, self.position - self.line_offset.1),
            );
        }

        None
    }
}
