/*  Copywrite Under MIT License by mahan farzaneh
 *
 *  TokenType: Has the type of every token supported py programming language
 *  Token: Turns Source code into An Iteration of tokens
 *
 * */
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /// Identifies a variable or functuin e.g: a, main, print
    Identifier,
    /// Numeric value e.g: 12 ,0xf3, 0b110
    Int(i32),
    /// Floating value e.g: 0.5
    Float(f64),
    /// Character Literal e.g: 'A', '9', '\n'
    Char(char),
    /// String Literal e.g: "Hello world", "hi\nhello"
    String,
    /// Inline Asm
    Plus,
    /// "-" Sub and Neg
    Minus,
    /// "*" Multiply
    Multi,
    /// ":" NOT DEFINED YET
    Colon,
    /// "," Seperating Arguments
    Comma,
    /// "["
    OBracket,
    /// "]"
    CBracket,
    /// END OF FILE
    Eof,
    /// START OF FILE
    Sof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Int(i) => write!(f, "{}", i),
            TokenType::Float(fl) => write!(f, "{}", fl),
            TokenType::Char(char) => write!(f, "{}", char),
            TokenType::String => write!(f, "String Literal"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Multi => write!(f, "*"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::OBracket => write!(f, "["),
            TokenType::CBracket => write!(f, "]"),
            TokenType::Eof => write!(f, "Eof"),
            TokenType::Sof => write!(f, "Sof"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub literal: String,
    pub t_type: TokenType,
}

impl Token {
    /// Returns a Token Structure
    ///
    /// # Arguments
    ///  
    /// * `t_type` - TokenType extracted by lexer
    /// * `literal` - The String Literal related to the token
    /// * `loc` - The location of the token
    ///
    /// # Examples
    ///
    /// ```
    /// Token::new(TokenType::Int(0),
    ///     "0".to_string(),
    ///     ("./path.nmt".to_string(),1,1)
    ///     );
    /// ```
    pub fn new(t_type: TokenType, literal: String) -> Self {
        Self {
            literal,
            t_type,
        }
    }

    /// Check if The Type of token is indicating the start
    /// or the end of a file
    pub fn is_empty(&self) -> bool {
        matches!(self.t_type, TokenType::Eof | TokenType::Sof)
    }

    /// Creates an empty token wich acts as None in Option<T>
    pub fn empty() -> Token {
        Self {
            literal: String::new(),
            t_type: TokenType::Sof,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub file_path: String,
    source: Vec<char>,
    pub token: Token,
    cur: usize,
    bol: usize,
    row: usize,
}

impl Lexer {
    /// Returns an instance of lexer
    ///
    /// # Argments
    ///
    /// * `file_path` - Path of code file mostly for error reporting
    /// * `source` - Code source String extracted from code file
    pub fn new(file_path: String, source: String) -> Self {
        Self {
            file_path,
            source: source.chars().collect::<Vec<char>>(),
            token: Token::empty(),
            cur: 0,
            bol: 0,
            row: 0,
        }
    }

    /// Chacks if self.cur is referencing outside of the code file
    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    /// Ignores Every character until a newline reached
    fn drop_line(&mut self) {
        while !self.is_empty() {
            if self.source[self.cur] == '\n' {
                self.drop();
                break;
            } else {
                self.drop();
            }
        }
    }

    /// Increments cur and adjusts bol and row if char under cur is a newline
    fn drop(&mut self) {
        if !self.is_empty() {
            let char = self.source[self.cur];
            self.cur += 1;
            if char == '\n' {
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    /// Drops all the whitespaces until reaching a non-WS char
    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_whitespace() {
            self.drop();
        }
    }

    /// Returns type of the current token
    /// Can exit the program if token is EOF
    pub fn get_token_type(&self) -> TokenType {
        let tk = self.token.clone();
        if tk.t_type == TokenType::Eof {
            panic!("Expected a Token, found Eof");
        };
        tk.t_type
    }

    /// Checks if the current token type matches the giver token type
    /// Will exit the program if token is not matching
    ///
    /// # Arguments
    ///
    /// * `t_type` - TokenType for matching
    pub fn match_token(&mut self, t_type: TokenType) {
        let tk = self.token.clone();
        if tk.t_type == t_type {
            self.next_token();
        } else {
            panic!(
                "Expected ({}), found ({})", t_type, tk.t_type,
            );
        }
    }

    /// Returns the current token
    pub fn get_token(&self) -> Token {
        self.token.clone()
    }


    /// Scans the next token and sets the current token to the new token
    pub fn next_token(&mut self) -> Token {
        let token = self._next_token();
        self.token = token.clone();
        token
    }

    /// Scans the next token
    fn _next_token(&mut self) -> Token {
        self.trim_left();
        while !self.is_empty() {
            if self.source[self.cur] == '~' {
                self.drop_line();
                self.trim_left();
            } else {
                break;
            }
        }
        if self.is_empty() {
            return Token::empty();
        }
        let first = self.source[self.cur];

        if first.is_ascii_alphabetic() || first == '_' {
            let index = self.cur;
            while !self.is_empty()
                && (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '_')
            {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
        }
        if first.is_ascii_digit() {
            let index = self.cur;
            self.drop();
            while !self.is_empty()
                && (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '.')
            {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            let ttype_and_val = self.parse_numeric_literal(&literal);
            return Token::new(ttype_and_val, literal);
        }
        if first == '\'' {
            return self.tokenize_char_literal();
        }

        if first == '"' {
            return self.tokenize_string_literal();
        }

        if let Some(tt) = Self::is_single_char_token(first) {
            self.drop();
            return Token::new(tt, first.to_string());
        }

        panic!("Unexpected Character");
    }

    /// Tokenses the char literal
    /// ONLY call when current char is (')
    fn tokenize_char_literal(&mut self) -> Token {
        self.drop();
        let literal;
        let char = self.source[self.cur];
        if char == '\'' {
            panic!("char literal can not be empty");
        }
        if char == '\\' {
            self.drop();
            if self.is_empty() {
                panic!("char literal unfinished escape sequence");
            }
            let escape = self.source[self.cur];
            match escape {
                'n' => {
                    literal = '\n';
                }
                '\'' => {
                    literal = '\'';
                }
                't' => {
                    literal = '\t';
                }
                'r' => {
                    literal = '\r';
                }
                '\\' => {
                    literal = '\\';
                }
                '0' => {
                    literal = '\\';
                }
                _ => {
                    panic!("unsupported escape sequence (\\{})", escape);
                }
            }
            self.drop();
        } else {
            literal = char;
            self.drop();
        }

        if !self.is_empty() {
            if self.source[self.cur] != '\'' {
                panic!("unsupported char");
            }
            self.drop();
            Token::new(TokenType::Char(literal), literal.to_string())
        } else {
            panic!("Error: Char literal is not closed properly");
        }
    }

    /// Tokenses the string literal
    /// ONLY call when current char is (")
    fn tokenize_string_literal(&mut self) -> Token {
        self.drop();
        let mut literal = String::new();
        while !self.is_empty() {
            let char = self.source[self.cur];
            if char == '\"' {
                break;
            }
            if char == '\n' {
                panic!("string literal not closed before end of line");
            }
            if char == '\\' {
                self.drop();
                if self.is_empty() {
                    panic!("string literal unfinished escape sequence");
                }

                let escape = self.source[self.cur];
                match escape {
                    'n' => {
                        literal.push('\n');
                        self.drop();
                    }
                    '"' => {
                        literal.push('"');
                        self.drop();
                    }
                    't' => {
                        literal.push('\t');
                        self.drop();
                    }
                    'r' => {
                        literal.push('\r');
                        self.drop();
                    }
                    '\\' => {
                        literal.push('\\');
                        self.drop();
                    }
                    _ => {
                        panic!("unsupported escape sequence (\\{})", escape);
                    }
                }
            } else {
                literal.push(char);
                self.drop();
            }
        }
        if !self.is_empty() {
            self.drop();
            Token::new(TokenType::String, literal)
        } else {
            panic!("Error: String literal is not closed properly");
        }
    }


    /// Checks if a char in literal is a token
    /// Returns Some(Token) if matches and None if not
    ///
    /// # Arguments
    ///
    /// * `literal` - token literal that we whant to check
    fn is_single_char_token(char: char) -> Option<TokenType> {
        match char {
            '[' => Some(TokenType::OBracket),
            ']' => Some(TokenType::CBracket),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '*' => Some(TokenType::Multi),
            ':' => Some(TokenType::Colon),
            ',' => Some(TokenType::Comma),
            _ => None,
        }
    }

    /// Parse numeric literal to a numeric TokenType
    /// Can Exit the program if can not parse the lietal
    ///
    /// # Arguments
    ///
    /// * `literal` - token literal that we whant to check
    fn parse_numeric_literal(&self, literal: &String) -> TokenType {
        let mut lit_chars = literal.chars();
        if literal.contains('x') {
            self.expect_char(&lit_chars.next(), vec!['0']);
            self.expect_char(&lit_chars.next(), vec!['x']);
            let mut value: i32 = 0;
            for ch in lit_chars {
                let digit = ch.to_digit(16).unwrap_or_else(|| {
                    panic!(
                        "Unknown character in parsing ({})", literal
                    );
                });
                value = (value * 16i32) + digit as i32;
            }
            TokenType::Int(value)
        } else if literal.contains('b') {
            self.expect_char(&lit_chars.next(), vec!['0']);
            self.expect_char(&lit_chars.next(), vec!['b']);
            let mut value: i32 = 0;
            for ch in lit_chars {
                let digit = ch.to_digit(2).unwrap_or_else(|| {
                    panic!(
                        "Unknown character in parsing ({})", literal
                    );
                });
                value = (value * 2i32) + digit as i32;
            }
            TokenType::Int(value)
        } else if literal.contains('.') {
            let value: f64 = literal.parse::<f64>().unwrap();
            TokenType::Float(value)
        } else {
            let value: i32 = literal.parse::<i32>().unwrap();
            TokenType::Int(value)
        }
    }

    /// Returns char if exits in a list
    /// Will Exit the program if no match
    fn expect_char(&self, copt: &Option<char>, chars: Vec<char>) -> char {
        let char = copt.unwrap_or_else(|| {
            panic!("Undifined character set for numbers");
        });
        if chars.contains(&char) {
            return char;
        }
        char
    }
}

