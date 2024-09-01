use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Token {
    Semicolon,
    LBracket,
    RBracket,
    If,
    Else,
    End,
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    If,
    Then,
    Else,
    Bracket,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.line().trim().to_string();
    let s = s.split_whitespace().collect::<Vec<&str>>();
    let mut stack = Vec::new();

    let process_open = |out: &mut io::BufWriter<io::StdoutLock<'_>>, stack: &mut Vec<State>| {
        if !stack.is_empty()
            && (stack.last() == Some(&State::If) || stack.last() == Some(&State::Else))
        {
            write!(out, "{{ ").unwrap();
        }
    };

    let process_close = |out: &mut io::BufWriter<io::StdoutLock<'_>>, stack: &mut Vec<State>| {
        while !stack.is_empty() && stack.last() == Some(&State::Else) {
            write!(out, "}} ").unwrap();
            stack.pop();
        }

        if !stack.is_empty() && stack.last() == Some(&State::If) {
            write!(out, "}} ").unwrap();
            if let Some(last) = stack.last_mut() {
                *last = State::Then;
            }
        }
    };

    for i in 0..s.len() {
        let token = match s[i] {
            ";" => Token::Semicolon,
            "{" => Token::LBracket,
            "}" => Token::RBracket,
            "if" => Token::If,
            "else" => Token::Else,
            "end" => Token::End,
            _ => unreachable!(),
        };

        match token {
            Token::Semicolon => {
                while !stack.is_empty() && stack.last() == Some(&State::Then) {
                    stack.pop();
                    process_close(&mut out, &mut stack);
                }

                process_open(&mut out, &mut stack);
                write!(out, "; ").unwrap();
                process_close(&mut out, &mut stack);
            }
            Token::LBracket => {
                write!(out, "{{ ").unwrap();
                stack.push(State::Bracket);
            }
            Token::RBracket => {
                while !stack.is_empty() && stack.last() == Some(&State::Then) {
                    stack.pop();
                    process_close(&mut out, &mut stack);
                }

                write!(out, "}} ").unwrap();
                stack.pop();

                if !stack.is_empty() {
                    if let Some(State::If) = stack.last() {
                        stack.pop();
                        stack.push(State::Then);
                    } else if let Some(State::Else) = stack.last() {
                        stack.pop();
                    }
                }

                process_close(&mut out, &mut stack);
            }
            Token::If => {
                while !stack.is_empty() && stack.last() == Some(&State::Then) {
                    stack.pop();
                    process_close(&mut out, &mut stack);
                }

                process_open(&mut out, &mut stack);
                write!(out, "if ").unwrap();
                stack.push(State::If);
            }
            Token::Else => {
                if let Some(last) = stack.last_mut() {
                    *last = State::Else;
                }

                write!(out, "else ").unwrap();
            }
            Token::End => {
                while !stack.is_empty() && stack.last() == Some(&State::Then) {
                    stack.pop();
                    process_close(&mut out, &mut stack);
                }

                while !stack.is_empty()
                    && (stack.last() == Some(&State::If) || stack.last() == Some(&State::Else))
                {
                    write!(out, "}} ").unwrap();
                    stack.pop();
                }

                writeln!(out, "end").unwrap();
                break;
            }
        }
    }
}
