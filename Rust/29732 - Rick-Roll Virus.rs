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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut is_treated = vec![false; n];

    for i in 0..n {
        if s[i] != 'R' {
            continue;
        }

        let left = i.saturating_sub(k);
        let right = (i + k).min(n - 1);

        for j in (left..=right).rev() {
            if is_treated[j] {
                break;
            }

            is_treated[j] = true;
        }
    }

    let cnt_treated = is_treated.iter().filter(|&&x| x).count();

    writeln!(out, "{}", if cnt_treated <= m { "Yes" } else { "No" }).unwrap();
}
