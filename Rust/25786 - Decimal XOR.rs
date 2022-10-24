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

    let (mut a, mut b) = (scan.token::<String>(), scan.token::<String>());

    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Less => {
            while a.len() < b.len() {
                a.insert(0, '0');
            }
        }
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Greater => {
            while a.len() > b.len() {
                b.insert(0, '0');
            }
        }
    }

    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();
    let mut ret = String::new();

    for i in 0..a.len() {
        if (a[i] <= '2' && b[i] <= '2') || (a[i] >= '7' && b[i] >= '7') {
            ret.push('0');
        } else {
            ret.push('9');
        }
    }

    writeln!(out, "{ret}").unwrap();
}
