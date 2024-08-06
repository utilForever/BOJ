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
    let mut positions = vec![0; n];
    let mut ret = Vec::new();

    for i in 0..n {
        positions[i] = scan.token::<i64>();
    }

    for i in 1..n {
        let mut checked = vec![false; n];
        let interval = positions[i] - positions[0];
        let mut left = 0;
        let mut right = 0;

        while left < n {
            while positions[right] - positions[left] < interval && right + 1 < n {
                right += 1;
            }

            if positions[right] - positions[left] == interval {
                checked[left] = true;
                checked[right] = true;
            }

            left += 1;
        }

        if checked.iter().filter(|&&x| x).count() == n {
            ret.push(interval);
        }
    }

    ret.sort();
    ret.dedup();

    writeln!(out, "{}", ret.len()).unwrap();

    if !ret.is_empty() {
        for interval in ret {
            write!(out, "{interval} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
