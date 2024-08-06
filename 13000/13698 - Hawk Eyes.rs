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

    let mut cups = [1, 0, 0, 2];

    let s = scan.token::<String>();

    for c in s.chars() {
        match c {
            'A' => {
                cups.swap(0, 1);
            }
            'B' => {
                cups.swap(0, 2);
            }
            'C' => {
                cups.swap(0, 3);
            }
            'D' => {
                cups.swap(1, 2);
            }
            'E' => {
                cups.swap(1, 3);
            }
            'F' => {
                cups.swap(2, 3);
            }
            _ => {}
        }
    }

    let pos_small_ball = cups.iter().position(|&x| x == 1).unwrap();
    let pos_big_ball = cups.iter().position(|&x| x == 2).unwrap();

    writeln!(out, "{}", pos_small_ball + 1).unwrap();
    writeln!(out, "{}", pos_big_ball + 1).unwrap();
}
