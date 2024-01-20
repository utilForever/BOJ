use io::Write;
use std::io;

#[derive(Clone, Default, Debug)]
struct Matrix {
    data: Vec<Vec<u16>>,
}

impl Matrix {
    fn new(data: Vec<Vec<u16>>) -> Self {
        Self { data }
    }

    fn push_column(&mut self, rhs: Self) -> Self {
        let mut ret = vec![vec![0; self.data[0].len() + rhs.data[0].len()]; self.data.len()];

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                ret[i][j] = self.data[i][j];
            }
        }

        for i in 0..rhs.data.len() {
            for j in 0..rhs.data[0].len() {
                ret[i][self.data[0].len() + j] = rhs.data[i][j];
            }
        }

        Matrix::new(ret)
    }

    fn push_row(&mut self, rhs: Self) -> Self {
        let mut ret = vec![vec![0; self.data[0].len()]; self.data.len() + rhs.data.len()];

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                ret[i][j] = self.data[i][j];
            }
        }

        for i in 0..rhs.data.len() {
            for j in 0..rhs.data[0].len() {
                ret[self.data.len() + i][j] = rhs.data[i][j];
            }
        }

        Matrix::new(ret)
    }

    fn transpose(&mut self) -> Self {
        let size_row = self.data.len();
        let size_col = self.data[0].len();
        let mut ret = vec![vec![0; size_row]; size_col];

        for i in 0..size_row {
            for j in 0..size_col {
                ret[j][i] = self.data[i][j];
            }
        }

        Matrix::new(ret)
    }

    fn index(&mut self, left: Self, right: Self) -> Self {
        let mut ret = vec![vec![0; right.data[0].len()]; left.data[0].len()];

        for i in 0..left.data[0].len() {
            for j in 0..right.data[0].len() {
                ret[i][j] = self.data[left.data[0][i] as usize - 1][right.data[0][j] as usize - 1];
            }
        }

        Matrix::new(ret)
    }
}

impl std::ops::Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = vec![vec![0; self.data[0].len()]; self.data.len()];

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                ret[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        Matrix::new(ret)
    }
}

impl std::ops::Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = vec![vec![0; self.data[0].len()]; self.data.len()];

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                ret[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }

        Matrix::new(ret)
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.data[0].len() == rhs.data.len() {
            let mut ret = vec![vec![0; rhs.data[0].len()]; self.data.len()];

            for i in 0..self.data.len() {
                for j in 0..rhs.data[0].len() {
                    for k in 0..self.data[0].len() {
                        ret[i][j] += self.data[i][k] * rhs.data[k][j];
                    }
                }
            }

            Matrix::new(ret)
        } else {
            let multiplier = if self.data.len() == 1 && self.data[0].len() == 1 {
                self.data.clone()
            } else {
                rhs.data.clone()
            };
            let mut ret = if self.data.len() == 1 && self.data[0].len() == 1 {
                rhs.data
            } else {
                self.data
            };

            for i in 0..ret.len() {
                for j in 0..ret[0].len() {
                    ret[i][j] *= multiplier[0][0];
                }
            }

            Matrix::new(ret)
        }
    }
}

impl std::ops::Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut ret = vec![vec![0; self.data[0].len()]; self.data.len()];

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                ret[i][j] = ret[i][j] - self.data[i][j];
            }
        }

        Matrix::new(ret)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Illegal,

    // Literals
    Ident(String),
    Int(u16),

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    SingleQuote,

    // Delimiters
    Whitespace,
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
        let tok = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'\'' => Token::SingleQuote,
            b' ' => Token::Whitespace,
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

        if !matches!(tok, Token::Ident(_) | Token::Int(_)) {
            self.read_char();
        }

        tok
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

        Token::Int(literal.parse::<u16>().unwrap())
    }
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::Eof,
        };

        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.curr_token = self.lexer.next_token();
    }

    fn curr_token_is(&mut self, token: Token) -> bool {
        self.curr_token == token
    }

    pub fn parse_program(&mut self, variables: &mut Vec<Matrix>) {
        // program ::= assignment | program assignment
        self.parse_assignment(variables);
    }

    pub fn parse_assignment(&mut self, variables: &mut Vec<Matrix>) {
        // assignment ::= var "=" expr "." NL
        let identifier = self.parse_identifier();

        self.next_token();
        self.next_token();

        let expression = self.parse_expression(variables);

        self.next_token();

        let identifier = identifier.chars().next().unwrap();
        variables[(identifier as u8 - 'A' as u8) as usize] = expression;
    }

    fn parse_identifier(&mut self) -> String {
        match &self.curr_token {
            Token::Ident(ident) => ident.clone(),
            _ => String::new(),
        }
    }

    fn parse_expression(&mut self, variables: &mut Vec<Matrix>) -> Matrix {
        // expr ::= term | expr "+" term | expr "-" term
        let mut left = self.parse_term(variables);

        while self.curr_token_is(Token::Plus) || self.curr_token_is(Token::Minus) {
            let is_plus = self.curr_token_is(Token::Plus);

            self.next_token();

            let right = self.parse_term(variables);

            left = if is_plus { left + right } else { left - right };
        }

        left
    }

    fn parse_term(&mut self, variables: &mut Vec<Matrix>) -> Matrix {
        // term ::= factor | term "*" factor
        let mut left = self.parse_factor(variables);

        while self.curr_token_is(Token::Asterisk) {
            self.next_token();

            let right = self.parse_factor(variables);

            left = left * right;
        }

        left
    }

    fn parse_factor(&mut self, variables: &mut Vec<Matrix>) -> Matrix {
        // factor ::= primary | "-" factor
        if self.curr_token_is(Token::Minus) {
            self.next_token();

            let ret = self.parse_factor(variables);

            return -ret;
        }

        self.parse_primary(variables)
    }

    fn parse_primary(&mut self, variables: &mut Vec<Matrix>) -> Matrix {
        // primary ::= inum | var | matrix | "(" expr ")" | indexed-primary | transposed-primary
        let mut ret = match self.curr_token.clone() {
            // inum ::= digit | inum digit
            // digit ::= "0" | "1" | ... | "9"
            Token::Int(num) => {
                self.next_token();

                Matrix::new(vec![vec![num]])
            }
            // var := "A" | "B" | ... | "Z"
            Token::Ident(ident) => {
                self.next_token();

                let identifier = ident.chars().next().unwrap();
                let matrix = variables[(identifier as u8 - 'A' as u8) as usize].clone();

                matrix
            }
            // indexed-primary ::= primary "(" expr "," expr ")"
            Token::Lparen => {
                self.next_token();

                let ret = self.parse_expression(variables);

                self.next_token();

                ret
            }
            // matrix ::= "[" row-seq "]"
            Token::Lbracket => {
                self.next_token();

                let ret = self.parse_row_seq(variables);

                self.next_token();

                ret
            }
            _ => Matrix::default(),
        };

        while self.curr_token_is(Token::Lparen) || self.curr_token_is(Token::SingleQuote) {
            if self.curr_token_is(Token::Lparen) {
                // indexed-primary ::= primary "(" expr "," expr ")"
                self.next_token();

                let expr_left = self.parse_expression(variables);

                self.next_token();

                let expr_right = self.parse_expression(variables);

                self.next_token();

                ret = ret.index(expr_left, expr_right);
            } else {
                // transposed-primary ::= primary "'"
                self.next_token();

                ret = ret.transpose();
            }
        }

        ret
    }

    fn parse_row_seq(&mut self, variables: &mut Vec<Matrix>) -> Matrix {
        // row-seq ::= row | row-seq ";" row
        let mut left = self.parse_row(variables);

        while self.curr_token_is(Token::Semicolon) {
            self.next_token();

            let right = self.parse_row(variables);

            left = left.push_row(right);
        }

        left
    }

    fn parse_row(&mut self, variables: &mut Vec<Matrix>) -> Matrix {
        // row ::= expr | row " " expr
        let mut left = self.parse_expression(variables);

        while self.curr_token_is(Token::Whitespace) {
            self.next_token();

            let right = self.parse_expression(variables);

            left = left.push_column(right);
        }

        left
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

        if n == 0 {
            continue;
        }

        let mut variables = vec![Matrix::default(); 26];

        for _ in 0..n {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();
            s = s.trim().to_string();

            let lexer = Lexer::new(&s);
            let mut parser = Parser::new(lexer);
            parser.parse_program(&mut variables);

            let ret = variables[s.chars().next().unwrap() as usize - 'A' as usize].clone();

            for i in 0..ret.data.len() {
                for j in 0..ret.data[0].len() {
                    write!(
                        out,
                        "{} ",
                        if ret.data[i][j] < 32768 {
                            ret.data[i][j]
                        } else {
                            ret.data[i][j] - 32768
                        }
                    )
                    .unwrap();
                }

                writeln!(out).unwrap();
            }
        }

        writeln!(out, "-----").unwrap();
    }
}
