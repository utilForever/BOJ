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
    let mut char_cur = 'A';
    let mut cnt_cur = 0_i64;
    let mut ret = 0_i64;

    for c in s.iter() {
        if *c as u8 == char_cur as u8 || *c as u8 == char_cur as u8 + 1 {
            // Count the number of 'A's in prefix
            if *c == 'A' {
                cnt_cur += 1;
            }

            // Count the number of 'Z's in suffix
            if *c == 'Z' {
                ret += cnt_cur;
            }

            char_cur = *c;
        } else {
            // Reset the counter
            cnt_cur = if *c == 'A' { 1 } else { 0 };
            char_cur = 'A';
        }
    }

    writeln!(out, "{ret}").unwrap();
}
