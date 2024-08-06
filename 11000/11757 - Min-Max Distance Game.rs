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

// Reference: https://www.slideshare.net/irrrrr/icpc-2015-tsukuba-unofficial-commentary
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, f) = (scan.token::<usize>(), scan.token::<String>());
    let mut dists = vec![0; n + 1];

    for i in 1..=n {
        dists[i] = scan.token::<i64>();
    }

    if (f == "Alice" && n % 2 == 1) || (f == "Bob" && n % 2 == 0) {
        let mut ret = dists[n];

        for i in 1..=n - (n + 1) / 2 {
            ret = ret.min(dists[i + (n + 1) / 2] - dists[i]);
        }

        writeln!(out, "{}", ret).unwrap();
        return;
    }

    let mut left = 0;
    let mut right = dists[n];
    let mut ret = 0;

    while left <= right {
        let mid = (left + right) / 2;
        let mut cnt = 1;
        let mut j = 1;

        for i in 1..=n {
            if dists[i] - dists[j] > mid {
                cnt += 1;
                j = i;
            }
        }

        if cnt > (n + 1) / 2 {
            left = mid + 1;
            ret = mid;
        } else {
            right = mid - 1;
        }
    }

    writeln!(out, "{}", ret + 1).unwrap();
}
