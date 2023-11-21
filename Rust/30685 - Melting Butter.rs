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
    let mut butters = vec![(0, 0); n];

    for i in 0..n {
        butters[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    butters.sort();

    let mut left = 1;
    let mut right = 2_000_000_000;
    let mut ret = i64::MAX;

    while left <= right {
        let mid = (left + right) / 2;
        let mut can_split = false;

        butters.windows(2).for_each(|butter| {
            if butter[0].0 + mid.min(butter[0].1 / 2) >= butter[1].0 - mid.min(butter[1].1 / 2) {
                can_split = true;
            }
        });

        if can_split {
            ret = ret.min(mid);
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    if ret == i64::MAX {
        writeln!(out, "forever").unwrap();
    } else {
        writeln!(out, "{}", ret - 1).unwrap();
    }
}
