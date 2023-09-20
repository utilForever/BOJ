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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, s) = (scan.token::<i64>(), scan.token::<String>());
    let s = s.chars().collect::<Vec<_>>();
    let mut ret_camel = s.clone();
    let mut ret_snake = s.clone();
    let mut ret_pascal = s.clone();

    if n == 1 {
        let mut idx = 0;

        while idx < ret_snake.len() {
            if ret_snake[idx].is_uppercase() {
                ret_snake[idx] = ret_snake[idx].to_ascii_lowercase();
                ret_snake.insert(idx, '_');
            }

            idx += 1;
        }

        ret_pascal[0] = ret_pascal[0].to_ascii_uppercase();
    } else if n == 2 {
        let mut idx = 0;

        while idx < ret_camel.len() {
            if ret_camel[idx] == '_' {
                ret_camel.remove(idx);
                ret_camel[idx] = ret_camel[idx].to_ascii_uppercase();
            }

            ret_pascal = ret_camel.clone();
            ret_pascal[0] = ret_pascal[0].to_ascii_uppercase();

            idx += 1;
        }
    } else {
        ret_camel[0] = ret_camel[0].to_ascii_lowercase();

        let mut idx = 0;
        ret_snake = ret_camel.clone();

        while idx < ret_snake.len() {
            if ret_snake[idx].is_uppercase() {
                ret_snake[idx] = ret_snake[idx].to_ascii_lowercase();
                ret_snake.insert(idx, '_');
            }

            idx += 1;
        }
    }

    writeln!(out, "{}", ret_camel.into_iter().collect::<String>()).unwrap();
    writeln!(out, "{}", ret_snake.into_iter().collect::<String>()).unwrap();
    writeln!(out, "{}", ret_pascal.into_iter().collect::<String>()).unwrap();
}
