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

    let is_palindrome = |s: &[char]| -> bool {
        let n = s.len();

        for i in 0..n / 2 {
            if s[i] != s[n - i - 1] {
                return false;
            }
        }

        true
    };

    let n = scan.token::<i64>();

    for i in 1..=n {
        let p = scan.token::<usize>();
        let mut conversions = vec![(String::new(), String::new()); p];

        for j in 0..p {
            conversions[j] = (scan.token::<String>(), scan.token::<String>());
        }

        writeln!(out, "Test case #{i}:").unwrap();

        let q = scan.token::<i64>();

        for _ in 0..q {
            let s = scan.token::<String>();
            let mut cloned = s.clone();

            for j in 0..p {
                cloned = cloned.replace(&conversions[j].0, &conversions[j].1);
            }

            writeln!(
                out,
                "{s} {}",
                if is_palindrome(&cloned.chars().collect::<Vec<char>>()) {
                    "YES"
                } else {
                    "NO"
                }
            )
            .unwrap();
        }

        if i != n {
            writeln!(out).unwrap();
        }
    }
}
