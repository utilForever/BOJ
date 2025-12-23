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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn is_valid(s: &Vec<char>) -> bool {
    let mut stack = Vec::new();

    for &c in s.iter() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' | '}' | ']' => {
                if let Some(top) = stack.pop() {
                    if !matches!((top, c), ('(', ')') | ('{', '}') | ('[', ']')) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }

    stack.is_empty()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    for _ in 0..n {
        let s = scan.token::<String>();
        let mut s = s.chars().collect::<Vec<_>>();

        if s.len() % 2 == 1 {
            writeln!(out, "NO").unwrap();
            continue;
        }

        if is_valid(&s) {
            writeln!(out, "YES 0").unwrap();
            continue;
        }

        let mut found = false;
        let mut pos = 0;
        let mut ch = ' ';

        'outer: for i in 0..s.len() {
            let original = s[i];

            for &c in ['(', ')', '{', '}', '[', ']'].iter() {
                if c == original {
                    continue;
                }

                s[i] = c;

                if is_valid(&s) {
                    found = true;
                    pos = i + 1;
                    ch = c;
                    break 'outer;
                }
            }

            s[i] = original;
        }

        if found {
            writeln!(out, "YES 1").unwrap();
            writeln!(out, "{pos} {ch}").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}
