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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut scores = vec![vec![0; n + 1]; 3];

        for i in 1..=n {
            scores[1][i] = scan.token::<i64>()
        }

        for i in 1..=n {
            scores[2][i] = scan.token::<i64>()
        }

        let (mut a, mut b, mut c, mut d) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if (a, b) > (c, d) {
            std::mem::swap(&mut a, &mut c);
            std::mem::swap(&mut b, &mut d);
        }

        let mut left = 0;
        let mut score = 0;

        for i in (1..=a - 1).rev() {
            left = left.max(score + scores[1][i].max(0) + scores[2][i].max(0));
            score += scores[1][i] + scores[2][i];
        }

        let mut mid = 0;

        for i in a + 1..=c - 1 {
            mid += scores[1][i]
                .max(scores[2][i])
                .max(scores[1][i] + scores[2][i]);
        }

        let mut right = 0;
        let mut score = 0;

        for i in c + 1..=n {
            right = right.max(score + scores[1][i].max(0) + scores[2][i].max(0));
            score += scores[1][i] + scores[2][i];
        }

        writeln!(
            out,
            "{}",
            if a == c {
                left.max(right) + scores[1][a] + scores[2][a]
            } else {
                let ret_left = (left + scores[1][a] + scores[2][a]).max(scores[b][a]);
                let ret_right = (right + scores[1][c] + scores[2][c]).max(scores[d][c]);

                ret_left + mid + ret_right
            }
        )
        .unwrap();
    }
}
