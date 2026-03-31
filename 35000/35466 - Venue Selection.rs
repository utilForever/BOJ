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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let (a, b, c) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (p, q) = (scan.token::<i64>(), scan.token::<i64>());

    let get_candidates = |dist: i64, n: i64| -> Vec<i64> {
        let a = dist % n;
        let b = (n - dist % n) % n;

        if a == b {
            vec![a]
        } else {
            vec![a, b]
        }
    };
    let get_dist = |x: i64, y: i64, n: i64| -> i64 {
        let diff = (x - y).abs();
        diff.min(n - diff)
    };

    let candidates_dgist = get_candidates(a, n);
    let candidates_postech = get_candidates(c, n);
    let candidates_hall = get_candidates(p, n);

    for &dgist in candidates_dgist.iter() {
        for &postech in candidates_postech.iter() {
            if get_dist(dgist, postech, n) != b {
                continue;
            }

            for &hall in candidates_hall.iter() {
                if get_dist(dgist, hall, n) != q {
                    continue;
                }

                writeln!(out, "{}", get_dist(postech, hall, n)).unwrap();
                return;
            }
        }
    }
}
