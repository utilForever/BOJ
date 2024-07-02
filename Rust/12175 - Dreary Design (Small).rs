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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let (k, v) = (scan.token::<i64>(), scan.token::<i64>());
        let mut ret = 0;

        // Attempt #1
        // for a in 0..=k {
        //     for b in 0..=k {
        //         for c in 0..=k {
        //             let min = a.min(b).min(c);
        //             let max = a.max(b).max(c);

        //             if max - min <= v {
        //                 ret += 1;
        //             }
        //         }
        //     }
        // }

        // Attempt #2
        // for a in 0..=k {
        //     let min = (a - v).max(0);
        //     let max = (a + v).min(k);

        //     for b in min..=max {
        //         let c = (b + v).min(max) - (b - v).max(min) + 1;
        //         ret += c;
        //     }
        // }

        // Attempt #3
        // for j in 0..=k {
        //     let min = (j - v).max(0);
        //     let max = (j + v).min(k);
        //     let delta = max - min + 1;
        //     let mut cnt = delta * delta;

        //     if delta > v {
        //         cnt -= (delta - v - 1) * (delta - v);
        //     }

        //     ret += cnt;
        // }

        // Part 1: j from 0 to v-1
        for j in 0..v {
            let delta = (j + v + 1).min(k + 1);
            let mut cnt = delta * delta;

            if delta > v {
                cnt -= (delta - v - 1) * (delta - v);
            }

            ret += cnt;
        }

        // Part 2: j from v to k-v
        if k >= 2 * v {
            let range = k - 2 * v + 1;
            let delta = 2 * v + 1;
            let mut cnt = delta * delta;

            if delta > v {
                cnt -= (delta - v - 1) * (delta - v);
            }

            ret += cnt * range;
        }

        // Part 3: j from k-v+1 to k
        for j in (k - v + 1).max(v)..=k {
            let delta = (k - j + v + 1).max(0);
            let mut cnt = delta * delta;

            if delta > v {
                cnt -= (delta - v - 1) * (delta - v);
            }

            ret += cnt;
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
