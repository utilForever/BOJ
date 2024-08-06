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

    let num = scan.token::<String>();
    let num = num.chars().map(|x| x.to_digit(10).unwrap() as i64).collect::<Vec<i64>>();

    for i in 0..num.len() {
        writeln!(
            out,
            "{}",
            match num[i] {
                0 => {
                    "0000\n0  0\n0  0\n0  0\n0000"
                }
                1 => {
                    "   1\n   1\n   1\n   1\n   1"
                }
                2 => {
                    "2222\n   2\n2222\n2\n2222"
                }
                3 => {
                    "3333\n   3\n3333\n   3\n3333"
                }
                4 => {
                    "4  4\n4  4\n4444\n   4\n   4"
                }
                5 => {
                    "5555\n5\n5555\n   5\n5555"
                }
                6 => {
                    "6666\n6\n6666\n6  6\n6666"
                }
                7 => {
                    "7777\n   7\n   7\n   7\n   7"
                }
                8 => {
                    "8888\n8  8\n8888\n8  8\n8888"
                }
                9 => {
                    "9999\n9  9\n9999\n   9\n   9"
                }
                _ => unreachable!(),
            }
        )
        .unwrap();
        writeln!(out).unwrap();
    }
}
