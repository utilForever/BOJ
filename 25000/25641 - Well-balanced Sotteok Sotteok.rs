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

    let _ = scan.token::<i64>();
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut cnt_s = s.iter().filter(|&&c| c == 's').count();
    let mut cnt_t = s.iter().filter(|&&c| c == 't').count();

    for (i, c) in s.iter().enumerate() {
        if cnt_s == cnt_t {
            writeln!(out, "{}", s[i..].iter().collect::<String>()).unwrap();
            break;
        }

        if *c == 's' {
            cnt_s -= 1;
        } else {
            cnt_t -= 1;
        }
    }
}
