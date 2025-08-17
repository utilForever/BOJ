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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = HashMap::new();

    for _ in 0..n {
        for _ in 0..m {
            let num = scan.token::<i64>();
            nums.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let mut cnt_pair = 0;

    for (_, v) in nums {
        cnt_pair += v / 2;
    }

    if m % 2 == 1 {
        writeln!(
            out,
            "{}",
            if cnt_pair >= n * (m - 1) / 2 {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    } else {
        writeln!(out, "{}", if cnt_pair >= n * m / 2 { "YES" } else { "NO" }).unwrap();
    }
}
