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

    let mut scores = [(0, 0); 8];

    for i in 0..8 {
        let score = scan.token::<i64>();
        scores[i] = (score, i + 1);
    }

    scores.sort_by(|a, b| b.0.cmp(&a.0));

    let mut ret = scores.iter().take(5).collect::<Vec<_>>();

    writeln!(out, "{}", ret.iter().map(|x| x.0).sum::<i64>()).unwrap();

    ret.sort_by(|a, b| a.1.cmp(&b.1));

    for score in ret {
        write!(out, "{} ", score.1).unwrap();
    }

    writeln!(out).unwrap();
}
