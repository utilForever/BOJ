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
    let mut time_curr = 0;
    let (mut score_a, mut score_b) = (0, 0);
    let (mut ret_a, mut ret_b) = (0, 0);

    for _ in 0..n {
        let (team, time) = (scan.token::<i64>(), scan.token::<String>());
        let time = time
            .split(":")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let time = time[0] * 60 + time[1];

        if score_a > score_b {
            ret_a += time - time_curr;
        } else if score_b > score_a {
            ret_b += time - time_curr;
        }

        if team == 1 {
            score_a += 1;
        } else {
            score_b += 1;
        }

        time_curr = time;
    }

    if score_a > score_b {
        ret_a += 48 * 60 - time_curr;
    } else if score_b > score_a {
        ret_b += 48 * 60 - time_curr;
    }

    writeln!(out, "{:02}:{:02}", ret_a / 60, ret_a % 60).unwrap();
    writeln!(out, "{:02}:{:02}", ret_b / 60, ret_b % 60).unwrap();
}
