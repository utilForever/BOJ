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

    for i in 1..=n {
        let m = scan.token::<i64>();
        let mut ret_tessa = 0;
        let mut ret_danny = 0;

        for _ in 0..m {
            let (score_tessa, score_danny) = (scan.token::<i64>(), scan.token::<i64>());

            if score_tessa - score_danny >= 2 {
                ret_tessa += score_tessa;
            } else if score_tessa - score_danny == 1 {
                ret_danny += if score_tessa == 2 && score_danny == 1 {
                    6
                } else {
                    score_tessa + score_danny
                };
            } else if score_danny - score_tessa == 1 {
                ret_tessa += if score_danny == 2 && score_tessa == 1 {
                    6
                } else {
                    score_tessa + score_danny
                };
            } else if score_danny - score_tessa >= 2 {
                ret_danny += score_danny;
            }
        }

        writeln!(out, "Game {i}: Tessa {ret_tessa} Danny {ret_danny}").unwrap();
    }
}
