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

fn parse(ason: &Vec<String>, idx: &mut usize) -> i64 {
    let mut ret = 0;

    *idx += 1;

    while ason[*idx] != "]" {
        let token = &ason[*idx];

        if token == "[" {
            let child = parse(ason, idx);
            ret += child;
        } else {
            if token.parse::<i64>().is_ok() {
                *idx += 1;
                ret += 8;
            } else {
                *idx += 1;
                ret += token.len() as i64 + 12;
            }
        }
    }

    *idx += 1;

    ret + 8
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let ason = scan.line().trim().to_string();
    let ason = ason
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut idx = 0;

    let ret = parse(&ason, &mut idx);

    writeln!(out, "{ret}").unwrap();
}
