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

    let n = scan.token::<usize>();
    let mut levels = vec![0; n];

    for i in 0..n {
        levels[i] = scan.token::<i64>();
    }

    levels.sort_by(|a, b| b.cmp(a));

    let mut sum_level = 0;
    let mut sum_ability = 0;

    for i in 0..n.min(42) {
        sum_level += levels[i];
        sum_ability += if levels[i] >= 250 {
            5
        } else if levels[i] >= 200 {
            4
        } else if levels[i] >= 140 {
            3
        } else if levels[i] >= 100 {
            2
        } else if levels[i] >= 60 {
            1
        } else {
            0
        };
    }

    writeln!(out, "{sum_level} {sum_ability}").unwrap();
}
