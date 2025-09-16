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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut golds = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                golds[i][j] = scan.token::<i32>();
            }
        }

        let size = 1 << n;
        let mut sum = vec![vec![0; size]; n];

        for mask in 1..size {
            let prev = mask & (mask - 1);
            let j = (mask ^ prev).trailing_zeros() as usize;

            for i in 0..n {
                sum[i][mask] = sum[i][prev] + golds[i][j];
            }
        }

        let mut visited = vec![false; size];
        let mut stack = Vec::new();
        let mut ret = vec![false; n];

        visited[size - 1] = true;
        stack.push(size - 1);

        while let Some(mask) = stack.pop() {
            if mask.count_ones() == 1 {
                ret[mask.trailing_zeros() as usize] = true;
                continue;
            }

            let mut m = mask;

            while m > 0 {
                let idx = m.trailing_zeros() as usize;
                let bit = 1 << idx;

                if sum[idx][mask] > 0 {
                    let next = mask ^ bit;

                    if !visited[next] {
                        visited[next] = true;
                        stack.push(next);
                    }
                }

                m &= m - 1;
            }
        }

        if ret.iter().all(|&x| !x) {
            writeln!(out, "0").unwrap();
            continue;
        }

        for i in 0..n {
            if ret[i] {
                write!(out, "{} ", i + 1).unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}
