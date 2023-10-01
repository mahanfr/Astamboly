use std::{fs, env::args};

mod elf;
mod instruct_table;

enum Intr {}

enum Instruct {
    Mov(Intr,Intr),
    Syscall,
}

enum Token {
    Mnemonic(String),
    Value(String),
    Comma,
    Colon,
    Hash,
    Eol,
}

struct Parser {
    source: Vec<char>,
    cur: usize,
    bol: usize,
    row: usize,
}
impl Parser {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            cur: 0,
            bol: 0,
            row: 0
        }
    }

    fn next_char(&mut self) {
        if self.source[self.cur] == '\n' || !self.is_empty() {
            self.row += 1;
            self.bol = 0;
        } else {
            self.bol += 1;
        }
        self.cur += 1;
    }

    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_whitespace() {
            self.next_char();
        }
    }

    fn next_token(&mut self) -> Token {
        self.trim_left();
        if self.source[self.cur] == ';' {
            while !self.is_empty() && self.source[self.cur] != '\n' {
                self.next_char();
            }
        }
        let cchar = self.source[self.cur];
        if cchar == '\n' {
            return Token::Eol;
        }
        if cchar.is_ascii_alphabetic() {
            let mut mnemonic = String::new();
            while !self.is_empty() && self.source[self.cur].is_ascii_alphabetic() {
                mnemonic.push(self.source[self.cur]);
                self.next_char();
            }
            return Token::Mnemonic(mnemonic);
        }
        if cchar.is_numeric() {
            let mut value = String::new();
            while !self.is_empty() && self.source[self.cur].is_ascii_alphanumeric() {
                value.push(self.source[self.cur]);
                self.next_char();
            }
            return Token::Value(value);
        }
        self.next_char();
        match cchar {
            ':' => Token::Colon,
            ',' => Token::Comma,
            '#' => Token::Hash,
            _ => panic!("Not Implemented yet"),
        }
    }
}


fn main() {
    let mut args = args();
    args.next().unwrap();
    let source = fs::read_to_string(args.next().unwrap()).expect("can not Read the file");
    println!("{}",source.len());
}
