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

    let _ = scan.token::<i64>();
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut ret = vec![String::new(); 5];

    for c in s {
        match c {
            'A' => {
                ret[0].push_str("***");
                ret[1].push_str("*.*");
                ret[2].push_str("***");
                ret[3].push_str("*.*");
                ret[4].push_str("*.*");
            }
            'B' => {
                ret[0].push_str("***");
                ret[1].push_str("*.*");
                ret[2].push_str("***");
                ret[3].push_str("*.*");
                ret[4].push_str("***");
            }
            'C' => {
                ret[0].push_str("***");
                ret[1].push_str("*..");
                ret[2].push_str("*..");
                ret[3].push_str("*..");
                ret[4].push_str("***");
            }
            'D' => {
                ret[0].push_str("***");
                ret[1].push_str("*.*");
                ret[2].push_str("*.*");
                ret[3].push_str("*.*");
                ret[4].push_str("***");
            }
            'E' => {
                ret[0].push_str("***");
                ret[1].push_str("*..");
                ret[2].push_str("***");
                ret[3].push_str("*..");
                ret[4].push_str("***");
            }
            _ => {}
        }
    }

    for i in 0..5 {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}
