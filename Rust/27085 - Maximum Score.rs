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

    let (n, s) = (scan.token::<usize>(), scan.token::<usize>());
    let mut scores = vec![0; n + 1];

    for i in 1..=n {
        scores[i] = scan.token::<i64>();
    }

    let mut left = s;
    let mut right = s;
    let mut score_left = 0;
    let mut score_right = 0;
    let mut should_escape = false;
    let mut ret = 0;

    while !should_escape {
        should_escape = true;

        while left > 1 && score_left + scores[left - 1] + ret >= 0 {
            left -= 1;
            score_left += scores[left];

            if score_left > 0 {
                ret += score_left;
                score_left = 0;
                should_escape = false;
                break;
            }
        }

        while right < n && score_right + scores[right + 1] + ret >= 0 {
            right += 1;
            score_right += scores[right];

            if score_right > 0 {
                ret += score_right;
                score_right = 0;
                should_escape = false;
                break;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
