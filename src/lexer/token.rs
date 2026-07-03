pub enum Token {
    //keywords
    Contract,
    If,
    Else,
    Function,
    Return,

    //literals
    Number(String),
    Identifier(String),
    StringLiteral(String),

    // Punctuation
    Semicolon,
    Comma,
    Dot,
    Colon, // 4

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace, //4

    // Operators
    Assign,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Multiply,
    Divide, // 7
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual, // 4, 11

    EOF,
}
