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

    for _ in 0..t {
        let n = scan.token::<usize>();
        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();

        if n == 1 || n == 3 {
            writeln!(out, "-1").unwrap();
            continue;
        }

        if n == 2 {
            writeln!(out, "{}", if s[0] == s[1] { "1" } else { "0" }).unwrap();
            continue;
        }

        let mut left = 1;
        let mut right = 1;

        while left < n && s[left] == s[left - 1] {
            left += 1;
        }

        while right < n && s[n - right] == s[n - right - 1] {
            right += 1;
        }

        if s[0] != s[n - 1] && left == right {
            writeln!(out, "0").unwrap();
        } else if s[0] != s[n - 1] && (left + right != n || (left as i64 - right as i64).abs() != 1)
            || (left.min(right) == 1 && left != right)
        {
            writeln!(out, "1").unwrap();
        } else {
            writeln!(out, "2").unwrap();
        }
    }
}
