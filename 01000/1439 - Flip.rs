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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut cnt_zero = 0;
    let mut cnt_one = 0;
    let mut is_one = if s[0] == '1' { true } else { false };

    for i in 1..s.len() {
        if s[i] == '1' {
            if !is_one {
                cnt_zero += 1;
                is_one = true;
            }
        } else {
            if is_one {
                cnt_one += 1;
                is_one = false;
            }
        }
    }

    if is_one && s[s.len() - 1] == '1' {
        cnt_one += 1;
    } else if !is_one && s[s.len() - 1] == '0' {
        cnt_zero += 1;
    }

    writeln!(out, "{}", cnt_zero.min(cnt_one)).unwrap();
}
