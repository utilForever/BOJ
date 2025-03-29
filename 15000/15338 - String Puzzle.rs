use io::Write;
use std::{collections::HashMap, io, str};

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

    let (n, a, b, q) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut letters = vec![(0, ' '); a];
    let mut hints = vec![(0, 0); b];

    for i in 0..a {
        let (x, c) = (scan.token::<i64>(), scan.token::<char>());
        letters[i] = (x, c);
    }

    for i in 0..b {
        let (y, h) = (scan.token::<i64>(), scan.token::<i64>());
        hints[i] = (y, h);
    }

    hints.push((n + 1, 0));

    let canonicalize = |hints: &Vec<(i64, i64)>, mut pos: i64| -> i64 {
        for i in (0..b).rev() {
            let left = hints[i].0;
            let right = hints[i + 1].0 - 1;

            if left > pos || right < pos {
                continue;
            }

            if hints[i].1 == 0 {
                return pos;
            }

            let cnt = (pos - left) / (hints[i].0 - hints[i].1);
            pos -= (cnt + 1) * (hints[i].0 - hints[i].1);
        }

        pos
    };
    let mut canonicalized = HashMap::new();

    for (x, c) in letters {
        let root = canonicalize(&hints, x);
        canonicalized.insert(root, c);
    }

    for _ in 0..q {
        let z = scan.token::<i64>();
        let root = canonicalize(&hints, z);
        write!(out, "{}", canonicalized.get(&root).unwrap_or(&'?')).unwrap();
    }

    writeln!(out).unwrap();
}
