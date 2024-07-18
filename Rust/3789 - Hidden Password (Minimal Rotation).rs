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

// Reference: https://github.com/jureslak/codebook/blob/master/implementacija/algo/minimal_rotation.cpp
fn minimal_rotation(s: &String) -> usize {
    let s = (s.clone() + &s).chars().collect::<Vec<_>>();
    let n = s.len();
    let mut i = 0;
    let mut j = 1;
    let mut k = 0;

    while i + k < n && j + k < n {
        if s[i + k] == s[j + k] {
            k += 1;
        } else if s[i + k] > s[j + k] {
            i += k + 1;

            if i <= j {
                i = j + 1;
            }

            k = 0;
        } else {
            j += k + 1;

            if j <= i {
                j = i + 1;
            }

            k = 0;
        }
    }

    i.min(j)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (_, s) = (scan.token::<usize>(), scan.token::<String>());
        writeln!(out, "{}", minimal_rotation(&s)).unwrap();
    }
}
