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

    let n = scan.token::<usize>();
    let mut scores = vec![(0, 0, 0); n];

    for i in 0..n {
        scores[i] = (i, scan.token::<i64>(), 0);
    }

    scores.sort_by(|a, b| b.1.cmp(&a.1));

    let mut score_prev = -1;
    let mut cnt_same_score = 0;
    let mut rank = 1;

    for score in scores.iter_mut() {
        if score.1 != score_prev {
            rank += cnt_same_score;
            cnt_same_score = 1;
        } else {
            cnt_same_score += 1;
        }

        score_prev = score.1;
        score.2 = rank;
    }

    scores.sort_by(|a, b| a.0.cmp(&b.0));

    for score in scores.iter() {
        writeln!(out, "{}", score.2).unwrap();
    }
}
