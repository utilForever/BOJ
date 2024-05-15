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

    let (_, q) = (scan.token::<i64>(), scan.token::<i64>());
    let s = scan.token::<String>();
    let mut s = s.chars().collect::<Vec<_>>();

    for _ in 0..q {
        let (command, l, r) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if command == 1 {
            let substring = s[l - 1..r].to_vec();
            let mut alphabet = substring[0];
            let mut ret = 1;

            for i in 1..substring.len() {
                if substring[i] != alphabet {
                    ret += 1;
                    alphabet = substring[i];
                }
            }

            writeln!(out, "{ret}").unwrap();
        } else {
            for i in l - 1..r {
                s[i] = ((s[i] as u8 - 'A' as u8 + 1) % 26 + 'A' as u8) as char;
            }
        }
    }
}
