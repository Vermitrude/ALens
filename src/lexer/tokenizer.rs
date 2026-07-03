use crate::lexer::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let current_char = input.chars().next();

        Self {
            input,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    pub fn next_token(&mut self) -> Token {
        if self.input.starts_with("contract") {
            return Token::Contract;
        }

        Token::EOF
    }
}
