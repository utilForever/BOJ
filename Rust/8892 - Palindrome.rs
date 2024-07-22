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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let is_palindrome = |s: &[char]| -> bool {
        let n = s.len();

        for i in 0..n / 2 {
            if s[i] != s[n - i - 1] {
                return false;
            }
        }

        true
    };

    for _ in 0..t {
        let k = scan.token::<usize>();
        let mut words = vec![String::new(); k];

        for i in 0..k {
            words[i] = scan.token::<String>();
        }

        let mut ret = None;

        'outer: for i in 0..k {
            for j in 0..k {
                if i == j {
                    continue;
                }

                let word = words[i].clone() + &words[j];

                if is_palindrome(&word.chars().collect::<Vec<char>>()) {
                    ret = Some(word);
                    break 'outer;
                }
            }
        }

        match ret {
            Some(word) => writeln!(out, "{word}").unwrap(),
            None => writeln!(out, "0").unwrap(),
        };
    }
}
