use io::Write;
use std::{cell::RefCell, collections::HashMap, fmt, io, rc::Rc, str};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    Illegal,

    // Identifiers + Literals
    Ident(String),
    Int(i64),

    // Operators
    Plus,
    Minus,
    Asterisk,

    // Delimiters
    Assign,
    Lparen,
    Rparen,

    // Reserved Keywords
    Print,
    Reset,
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
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b':' => {
                self.read_char();
                Token::Assign
            }
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'a'..=b'z' | b'A'..=b'Z' => {
                return self.read_identifier();
            }
            b'0'..=b'9' => {
                return self.read_number();
            }
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
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[position..self.position];

        match literal {
            "PRINT" => Token::Print,
            "RESET" => Token::Reset,
            _ => Token::Ident(String::from(literal)),
        }
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

#[derive(Debug, Clone, PartialEq)]
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
}

impl std::fmt::Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Multiply => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assign(Identifier, Expression),
    Print(Expression),
    Reset,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
}

pub type Program = Vec<Statement>;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Sum,     // +
    Product, // *
    Prefix,  // -X
    Group,   // (X)
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::Eof,
            peek_token: Token::Eof,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn token_to_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk => Precedence::Product,
            Token::Lparen => Precedence::Group,
            _ => Precedence::Lowest,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn peek_token_is(&mut self, token: Token) -> bool {
        self.peek_token == token
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token_is(token.clone()) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn curr_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.curr_token)
    }

    fn peek_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.peek_token)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();

        while self.curr_token != Token::Eof {
            match self.parse_statement() {
                Some(statement) => program.push(statement),
                None => {}
            }

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        // <statement> ::= <assignment> | <print> | <reset>
        match self.curr_token {
            Token::Ident(_) => self.parse_assignment_statement(),
            Token::Print => self.parse_print_statment(),
            Token::Reset => self.parse_reset_statement(),
            _ => None,
        }
    }

    fn parse_assignment_statement(&mut self) -> Option<Statement> {
        // <assignment> ::= <var> ":=" <expr>
        let identifier = match self.parse_identifier() {
            Some(identifier) => identifier,
            None => return None,
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();

        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expression) => expression,
            None => return None,
        };

        Some(Statement::Assign(identifier, expression))
    }

    fn parse_print_statment(&mut self) -> Option<Statement> {
        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expression) => expression,
            None => return None,
        };

        Some(Statement::Print(expression))
    }

    fn parse_reset_statement(&mut self) -> Option<Statement> {
        Some(Statement::Reset)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Prefix
        let mut left = match self.curr_token {
            Token::Ident(_) => self.parse_identifier_expression(),
            Token::Int(_) => self.parse_int_expression(),
            Token::Minus => self.parse_prefix_expression(),
            Token::Lparen => self.parse_grouped_expression(),
            _ => None,
        };

        // Infix
        while precedence < self.peek_token_precedence() {
            match self.peek_token {
                Token::Plus | Token::Minus | Token::Asterisk => {
                    self.next_token();
                    left = self.parse_infix_expression(left.unwrap());
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        match &self.curr_token {
            Token::Ident(ident) => Some(Identifier(ident.clone())),
            _ => None,
        }
    }

    fn parse_identifier_expression(&mut self) -> Option<Expression> {
        match self.parse_identifier() {
            Some(ident) => Some(Expression::Identifier(ident)),
            None => None,
        }
    }

    fn parse_int_expression(&mut self) -> Option<Expression> {
        match &self.curr_token {
            Token::Int(int) => Some(Expression::Literal(Literal::Int(int.clone()))),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let prefix = match self.curr_token {
            Token::Minus => Prefix::Minus,
            _ => return None,
        };

        self.next_token();

        match self.parse_expression(Precedence::Prefix) {
            Some(expr) => Some(Expression::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let infix = match self.curr_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Asterisk => Infix::Multiply,
            _ => return None,
        };

        let precedence = self.curr_token_precedence();

        self.next_token();

        match self.parse_expression(precedence) {
            Some(expr) => Some(Expression::Infix(infix, Box::new(left), Box::new(expr))),
            None => None,
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(Token::Rparen) {
            None
        } else {
            expr
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Error(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Int(ref value) => write!(f, "{value}"),
            Object::Error(ref value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_with_outer(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
        match self.store.get(&name) {
            Some(value) => Some(value.clone()),
            None => match self.outer {
                Some(ref outer) => outer.borrow_mut().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, value: &Object) {
        self.store.insert(name, value.clone());
    }
}

pub struct Evaluator {
    environment: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Evaluator { environment }
    }

    fn error(msg: String) -> Object {
        Object::Error(msg)
    }

    fn is_error(object: &Object) -> bool {
        match object {
            Object::Error(_) => true,
            _ => false,
        }
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;

        for statement in program {
            match self.eval_statement(statement) {
                Some(Object::Error(msg)) => return Some(Object::Error(msg)),
                object => result = object,
            }
        }

        result
    }

    fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        match statement {
            Statement::Assign(identifier, expression) => {
                let value = match self.eval_expression(expression) {
                    Some(value) => value,
                    None => return None,
                };

                if Self::is_error(&value) {
                    Some(value)
                } else {
                    let Identifier(name) = identifier;
                    self.environment.borrow_mut().set(name, &value);

                    None
                }
            }
            Statement::Print(expression) => {
                let value = match self.eval_expression(expression) {
                    Some(value) => value,
                    None => return None,
                };

                Some(value)
            }
            Statement::Reset => {
                self.environment.borrow_mut().store.clear();
                None
            }
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Option<Object> {
        match expression {
            Expression::Identifier(identifier) => Some(self.eval_identifier(identifier)),
            Expression::Literal(literal) => Some(self.eval_literal(literal)),
            Expression::Prefix(prefix, right_expression) => {
                if let Some(right) = self.eval_expression(*right_expression) {
                    Some(self.eval_prefix_expression(prefix, right))
                } else {
                    None
                }
            }
            Expression::Infix(infix, left_expression, right_expression) => {
                let left = self.eval_expression(*left_expression);
                let right = self.eval_expression(*right_expression);

                if left.is_some() && right.is_some() {
                    Some(self.eval_infix_expression(infix, left.unwrap(), right.unwrap()))
                } else {
                    None
                }
            }
        }
    }

    fn eval_identifier(&mut self, identifier: Identifier) -> Object {
        let Identifier(name) = identifier;

        match self.environment.borrow_mut().get(name.clone()) {
            Some(value) => value,
            None => Object::Error(String::from(format!("identifier not found: {name}"))),
        }
    }

    fn eval_literal(&mut self, literal: Literal) -> Object {
        match literal {
            Literal::Int(value) => Object::Int(value),
        }
    }

    fn eval_prefix_expression(&mut self, prefix: Prefix, right: Object) -> Object {
        match prefix {
            Prefix::Minus => self.eval_minus_prefix_expression(right),
        }
    }

    fn eval_minus_prefix_expression(&mut self, right: Object) -> Object {
        match right {
            Object::Int(value) => Object::Int(-value),
            _ => Self::error(format!("unknown operator: -{right}")),
        }
    }

    fn eval_infix_expression(&mut self, infix: Infix, left: Object, right: Object) -> Object {
        match left {
            Object::Int(left_value) => {
                if let Object::Int(right_value) = right {
                    self.eval_infix_integer_expression(infix, left_value, right_value)
                } else {
                    Self::error(format!("type mismatch: {left} {infix} {right}"))
                }
            }
            _ => Self::error(format!("unknown operator: {left} {infix} {right}")),
        }
    }

    fn eval_infix_integer_expression(
        &mut self,
        infix: Infix,
        left_value: i64,
        right_value: i64,
    ) -> Object {
        match infix {
            Infix::Plus => Object::Int(left_value + right_value),
            Infix::Minus => Object::Int(left_value - right_value),
            Infix::Multiply => Object::Int(left_value * right_value),
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let environment = Environment::new();
    let mut evaluator = Evaluator::new(Rc::new(RefCell::new(environment)));

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let mut parser = Parser::new(Lexer::new(&s));
        let program = parser.parse_program();

        if let Some(evaluated) = evaluator.eval(program) {
            writeln!(out, "{evaluated}").unwrap();
        } else {
            writeln!(out, "UNDEF").unwrap();
        }
    }
}
