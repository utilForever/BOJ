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

    let mut score_w = vec![0; 10];
    let mut score_l = vec![0; 10];

    for i in 0..10 {
        score_w[i] = scan.token::<i64>();
    }

    for i in 0..10 {
        score_l[i] = scan.token::<i64>();
    }

    score_w.sort_by(|a, b| b.cmp(a));
    score_l.sort_by(|a, b| b.cmp(a));

    writeln!(
        out,
        "{} {}",
        score_w[0] + score_w[1] + score_w[2],
        score_l[0] + score_l[1] + score_l[2]
    )
    .unwrap();
}
