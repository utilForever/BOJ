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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut sundews = vec![(0, 0, 0); m + 1];

    for i in 0..m {
        sundews[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    sundews[m] = (0, n, -1);
    sundews.sort_by(|a, b| a.1.cmp(&b.1));

    let mut pos_y = 0;
    let mut ret = true;

    for i in 0..=m {
        let (c, x, h) = sundews[i];

        if c == 0 {
            if pos_y <= h {
                pos_y = h + 1;
            } else if pos_y - 1 > h {
                let diff = if i == 0 { x } else { x - sundews[i - 1].1 };
                pos_y -= diff;
            }
        } else {
            let diff = if i == 0 { x } else { x - sundews[i - 1].1 };
            pos_y -= diff;

            if pos_y >= h {
                ret = false;
                break;
            }
        }
    }

    if pos_y != 0 {
        ret = false;
    }

    writeln!(out, "{}", if ret { "stay" } else { "adios" }).unwrap();
}
