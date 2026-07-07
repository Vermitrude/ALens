use crate::lexer::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.first().copied();

        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        number
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();

        // Skip opening quote
        self.advance();

        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            }

            string.push(c);
            self.advance();
        }

        // Skip closing quote
        self.advance();

        string
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            Some('{') => {
                self.advance();
                Token::LeftBrace
            }

            Some('}') => {
                self.advance();
                Token::RightBrace
            }

            Some('(') => {
                self.advance();
                Token::LeftParen
            }

            Some(')') => {
                self.advance();
                Token::RightParen
            }

            Some(';') => {
                self.advance();
                Token::Semicolon
            }

            Some(',') => {
                self.advance();
                Token::Comma
            }

            Some('.') => {
                self.advance();
                Token::Dot
            }

            Some(':') => {
                self.advance();
                Token::Colon
            }

            Some('+') => {
                self.advance();
                Token::Plus
            }

            Some('-') => {
                self.advance();
                Token::Minus
            }

            Some('*') => {
                self.advance();
                Token::Multiply
            }

            Some('/') => {
                self.advance();
                Token::Divide
            }

            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    self.advance();
                    Token::Equal
                } else {
                    self.advance();
                    Token::Assign
                }
            }

            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    self.advance();
                    Token::NotEqual
                } else {
                    self.advance();
                    Token::Illegal
                }
            }

            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    self.advance();
                    Token::GreaterThanOrEqual
                } else {
                    self.advance();
                    Token::GreaterThan
                }
            }

            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    self.advance();
                    Token::LessThanOrEqual
                } else {
                    self.advance();
                    Token::LessThan
                }
            }

            Some('"') => {
                Token::StringLiteral(self.read_string())
            }

            Some(c) if c.is_ascii_digit() => {
                Token::Number(self.read_number())
            }

            Some(c) if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier();

                match ident.as_str() {
                    "contract" => Token::Contract,
                    "function" => Token::Function,
                    "pragma" => Token::Pragma,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Identifier(ident),
                }
            }

            None => Token::EOF,

            _ => {
                self.advance();
                Token::Illegal
            }
        }
    }
}