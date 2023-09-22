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

    let n = scan.token::<i64>();
    let mut sum_mari = 0;
    let mut sum_juri = 0;
    let mut cnt_mari = 0;
    let mut cnt_juri = 0;
    let mut score_mari = 0.0;
    let mut score_juri = 0.0;

    for _ in 0..n {
        let (player, score) = (scan.token::<char>(), scan.token::<i64>());

        if player == 'M' {
            sum_mari += score;
            cnt_mari += 1;
        } else {
            sum_juri += score;
            cnt_juri += 1;
        }
    }

    if cnt_mari > 0 {
        score_mari = sum_mari as f64 / cnt_mari as f64;
    }

    if cnt_juri > 0 {
        score_juri = sum_juri as f64 / cnt_juri as f64;
    }

    writeln!(
        out,
        "{}",
        if score_mari > score_juri {
            "M"
        } else if score_mari < score_juri {
            "J"
        } else {
            "V"
        }
    )
    .unwrap();
}
