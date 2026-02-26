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
        let (n, k) = (scan.token::<i64>(), scan.token::<i64>());

        if n < 2 * k || n > 4 * k + 1 {
            writeln!(out, "NO").unwrap();
            continue;
        }

        writeln!(out, "YES").unwrap();

        for i in 1..2 * k {
            writeln!(out, "{i} 0").unwrap();
        }

        let remain = n - 2 * k;

        if remain > 0 {
            let mut candidates = Vec::new();

            candidates.push((-1, 0));
            candidates.push((0, 1));
            candidates.push((0, -1));

            for i in 1..k {
                candidates.push((2 * i, 1));
                candidates.push((2 * i, -1));
            }

            for i in 0..remain as usize {
                writeln!(out, "{} {}", candidates[i].0, candidates[i].1).unwrap();
            }
        }
    }
}
