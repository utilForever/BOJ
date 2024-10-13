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

    let t = scan.token::<i64>();
    let mut fair_and_square_numbers = vec![false; 1001];
    let is_palindrome = |x: i64| -> bool {
        let s = x.to_string();
        s == s.chars().rev().collect::<String>()
    };

    for i in 1..=1000 {
        if i * i > 1000 {
            break;
        }

        fair_and_square_numbers[(i * i) as usize] = is_palindrome(i) && is_palindrome(i * i);
    }

    for i in 1..=t {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        let mut ret = 0;

        for j in a..=b {
            if fair_and_square_numbers[j] {
                ret += 1;
            }
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
