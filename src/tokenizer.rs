use crate::token::{Keyword, Token};

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(ch) = self.peek_char() {
            self.position += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.next_char() {
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let mut ident = ch.to_string();
                while let Some(next) = self.peek_char() {
                    if next.is_alphanumeric() || next == '_' {
                        ident.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }

                match ident.to_uppercase().as_str() {
                    "SELECT" => Token::Keyword(Keyword::Select),
                    "CREATE" => Token::Keyword(Keyword::Create),
                    "TABLE" => Token::Keyword(Keyword::Table),
                    "WHERE" => Token::Keyword(Keyword::Where),
                    "ORDER" => Token::Keyword(Keyword::Order),
                    "BY" => Token::Keyword(Keyword::By),
                    "ASC" => Token::Keyword(Keyword::Asc),
                    "DESC" => Token::Keyword(Keyword::Desc),
                    "FROM" => Token::Keyword(Keyword::From),
                    "AND" => Token::Keyword(Keyword::And),
                    "OR" => Token::Keyword(Keyword::Or),
                    "NOT" => Token::Keyword(Keyword::Not),
                    "TRUE" => Token::Keyword(Keyword::True),
                    "FALSE" => Token::Keyword(Keyword::False),
                    "PRIMARY" => Token::Keyword(Keyword::Primary),
                    "KEY" => Token::Keyword(Keyword::Key),
                    "CHECK" => Token::Keyword(Keyword::Check),
                    "INT" => Token::Keyword(Keyword::Int),
                    "BOOL" => Token::Keyword(Keyword::Bool),
                    "VARCHAR" => Token::Keyword(Keyword::Varchar),
                    "NULL" => Token::Keyword(Keyword::Null),
                    _ => Token::Identifier(ident),
                }
            }
            Some(ch) if ch.is_ascii_digit() => {
                let mut number = ch.to_string();
                while let Some(next) = self.peek_char() {
                    if next.is_ascii_digit() {
                        number.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(number.parse().unwrap())
            }
            Some('\'') => {
                let mut s = String::new();
                while let Some(ch) = self.next_char() {
                    if ch == '\'' {
                        break;
                    }
                    s.push(ch);
                }
                Token::String(s)
            }
            Some('=') => Token::Equal,
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.next_char();
                    Token::NotEqual
                } else {
                    Token::Invalid('!')
                }
            }
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.next_char();
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.next_char();
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                }
            }
            Some('*') => Token::Star,
            Some('/') => Token::Divide,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LeftParentheses,
            Some(')') => Token::RightParentheses,
            Some(c) => Token::Invalid(c),
            None => Token::Eof,
        }
    }
}

