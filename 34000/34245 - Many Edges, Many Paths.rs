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

const FULL_D: usize = 99;
const PARTIAL_D: usize = 100;
const PARTIAL_CNT: usize = 98;

fn make_block(
    out: &mut io::BufWriter<std::io::StdoutLock>,
    visited: &mut Vec<bool>,
    start: &mut usize,
    n: usize,
    d: usize,
    r: usize,
) {
    let mut cursor = (0..d)
        .map(|val| (n - 1) - ((n - 1 - val) % d))
        .collect::<Vec<_>>();

    for _ in 0..r {
        let s = *start;
        let remain = s % d;
        let mut e = cursor[remain];

        while e >= 2 && visited[e] {
            if e < d {
                break;
            }

            e -= d;
        }

        visited[e] = true;
        cursor[remain] = if e >= d { e - d } else { 0 };
        *start += 1;

        let steps = if e >= s { (e - s) / d } else { 0 };
        let cnt = steps + 3;

        write!(out, "{cnt} ").unwrap();
        write!(out, "1 {s} ").unwrap();

        let mut val = s;

        while val + d <= e {
            val += d;
            write!(out, "{val} ").unwrap();
        }

        writeln!(out, "{n}").unwrap();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, _) = (scan.token::<usize>(), scan.token::<i64>());

    writeln!(out, "{}", FULL_D * (FULL_D + 1) / 2 + PARTIAL_CNT).unwrap();

    let mut visited = vec![false; n + 1];
    let mut start = 2;

    for i in 1..=FULL_D {
        make_block(&mut out, &mut visited, &mut start, n, i, i);
    }

    make_block(
        &mut out,
        &mut visited,
        &mut start,
        n,
        PARTIAL_D,
        PARTIAL_CNT,
    );
}
