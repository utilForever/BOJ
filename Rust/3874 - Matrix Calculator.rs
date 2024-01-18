use io::Write;
use std::{io, fmt};

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Multiply,
    Transpose,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    String(String),
    Int(i64),
    Matrix(Vec<Expression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assignment(Identifier, Expression),
}

pub type Program = Vec<Statement>;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Illegal,

    // Literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    SingleQuote,

    // Delimiters
    Comma,
    Semicolon,
    Dot,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'\'' => Token::SingleQuote,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'.' => Token::Dot,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'[' => Token::Lbracket,
            b']' => Token::Rbracket,
            b'A'..=b'Z' => self.read_identifier(),
            b'0'..=b'9' => self.read_number(),
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        tok
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;

        loop {
            match self.ch {
                b'A'..=b'Z' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[position..self.position];

        Token::Ident(String::from(literal))
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;

        loop {
            match self.ch {
                b'0'..=b'9' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[position..self.position];

        Token::Int(literal.parse::<i64>().unwrap())
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorKind::UnexpectedToken => write!(f, "Unexpected Token"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ParseErrorKind,
    msg: String,
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();

        while self.cur_token != Token::Eof {
            match self.parse_statement() {
                Some(statement) => program.push(statement),
                None => {}
            }

            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Ident(_) => self.parse_assignment_statement(),
            _ => None,
        }
    }

    pub fn parse_assignment_statement(&mut self) -> Option<Statement> {
        
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let n = s.parse::<i64>().unwrap();

        for _ in 0..n {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();
            s = s.trim().to_string();

            let input = s.chars().collect::<Vec<_>>();
            let mut lexer = Lexer::new(&s[2..]);
            let mut parser = Parser::new(lexer);
        }
    }
}
