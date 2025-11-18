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

    let mut is_prime = vec![true; 100000];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(99999 as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=99999).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut prefix_sum = vec![0; 100000];

    for i in 1..100000 {
        prefix_sum[i] = prefix_sum[i - 1] + if is_prime[i] { 1 } else { 0 };
    }

    let n = scan.token::<usize>();
    let mut score_max = (i64::MIN, String::new());
    let mut score_min = (i64::MAX, String::new());

    for _ in 0..n {
        let (name, num) = (scan.token::<String>(), scan.token::<usize>());
        let (mut left, mut right) = (num / 100000, num % 100000);

        if left > right {
            std::mem::swap(&mut left, &mut right);
        }

        let score = prefix_sum[right] - prefix_sum[left - 1];

        if score > score_max.0 || (score == score_max.0 && name < score_max.1) {
            score_max = (score, name.clone());
        }

        if score < score_min.0 || (score == score_min.0 && name < score_min.1) {
            score_min = (score, name.clone());
        }
    }

    writeln!(out, "{}", score_max.1).unwrap();
    writeln!(out, "{}", score_min.1).unwrap();
}
