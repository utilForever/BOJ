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

    let n = scan.token::<i64>();
    let mut ret = Vec::new();

    for _ in 0..n {
        let paper = scan.token::<String>();
        let paper = paper.chars().collect::<Vec<_>>();
        let mut idx = 0;
        let mut number = String::new();

        while idx < paper.len() {
            if paper[idx].is_numeric() {
                number.push(paper[idx]);
            } else {
                if number.len() > 0 {
                    let mut num = number.trim_start_matches('0').to_string();
                    if num.len() == 0 {
                        num.push('0');
                    }

                    ret.push(num.parse::<String>().unwrap());
                    number.clear();
                }
            }

            idx += 1;
        }

        if number.len() > 0 {
            let mut num = number.trim_start_matches('0').to_string();
            if num.len() == 0 {
                num.push('0');
            }

            ret.push(num.parse::<String>().unwrap());
        }
    }

    ret.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
