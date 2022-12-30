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

    loop {
        let mut paper1 = [0; 2];
        let mut paper2 = [0; 2];

        (paper1[0], paper1[1], paper2[0], paper2[1]) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if paper1[0] == 0 && paper1[1] == 0 && paper2[0] == 0 && paper2[1] == 0 {
            break;
        }

        paper1.sort();
        paper2.sort();

        let mut percent = 0_i64;

        loop {
            let len1 = paper1[0] as f64 * (100.0 - percent as f64) / 100.0;
            let len2 = paper1[1] as f64 * (100.0 - percent as f64) / 100.0;

            if len1 <= paper2[0] as f64 && len2 <= paper2[1] as f64 {
                break;
            } else {
                percent += 1;
            }
        }

        writeln!(out, "{}%", 100 - percent).unwrap();
    }
}
