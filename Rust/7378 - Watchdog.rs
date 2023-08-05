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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (s, h) = (scan.token::<i64>(), scan.token::<usize>());
        let mut hatches = vec![(0, 0); h];
        let mut ret = None;

        for i in 0..h {
            hatches[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        for x in 1..s {
            for y in 1..s {
                if hatches.contains(&(x, y)) {
                    continue;
                }

                let dist_min = x.min(s - x).min(y).min(s - y) as f64;

                if hatches.iter().all(|hatch| {
                    let dist = ((x - hatch.0) as f64).hypot((y - hatch.1) as f64);
                    dist <= dist_min
                }) {
                    ret = Some((x, y));
                    break;
                }
            }

            if ret.is_some() {
                break;
            }
        }

        match ret {
            Some((x, y)) => writeln!(out, "{x} {y}").unwrap(),
            None => writeln!(out, "poodle").unwrap(),
        }
    }
}
