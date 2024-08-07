use io::Write;
use std::{collections::HashSet, io, str};

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
    let m = scan.token::<usize>();
    let mut combinations = HashSet::new();

    for _ in 0..m {
        let a = scan.token::<i64>();
        let b: i64 = scan.token::<i64>();
        combinations.insert(if a < b { (a, b) } else { (b, a) });
    }

    let mut ret = 0;

    for i in 1..=n - 2 {
        for j in i + 1..=n - 1 {
            if combinations.contains(&(i, j)) {
                continue;
            }

            for k in j + 1..=n {
                if combinations.contains(&(j, k)) || combinations.contains(&(i, k)) {
                    continue;
                }

                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
