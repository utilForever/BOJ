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

    let t = scan.token::<usize>();
    let mut scores = [0; 5];

    for i in 0..t {
        scores[i] = scan.token::<i64>();
    }

    let mut ret = 0;
    ret += if scores[0] > scores[2] {
        (scores[0] - scores[2]) * 508
    } else {
        (scores[2] - scores[0]) * 108
    };
    ret += if scores[1] > scores[3] {
        (scores[1] - scores[3]) * 212
    } else {
        (scores[3] - scores[1]) * 305
    };
    ret += if scores[4] > 0 { scores[4] * 707 } else { 0 };
    ret *= 4763;

    writeln!(out, "{ret}").unwrap();
}
