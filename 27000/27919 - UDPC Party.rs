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

    let v = scan.token::<String>();
    let votes = v.chars().collect::<Vec<_>>();
    let mut cnt_u = 0;
    let mut cnt_d = 0;
    let mut cnt_p = 0;
    let mut cnt_c = 0;

    for vote in votes {
        match vote {
            'U' => cnt_u += 1,
            'D' => cnt_d += 1,
            'P' => cnt_p += 1,
            'C' => cnt_c += 1,
            _ => (),
        }
    }

    if cnt_u + cnt_c > (cnt_d + cnt_p + 1) / 2 {
        write!(out, "U").unwrap();
    }

    if cnt_d + cnt_p > 0 {
        write!(out, "DP").unwrap();
    }

    writeln!(out).unwrap();
}
