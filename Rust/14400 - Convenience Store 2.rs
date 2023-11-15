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
    let mut pos_x = vec![0; n];
    let mut pos_y = vec![0; n];

    for i in 0..n {
        pos_x[i] = scan.token::<i64>();
        pos_y[i] = scan.token::<i64>();
    }

    pos_x.sort();
    pos_y.sort();

    if n % 2 == 0 {
        let ret1_x = pos_x[n / 2 - 1];
        let ret1_y = pos_y[n / 2 - 1];
        let ret2_x = pos_x[n / 2];
        let ret2_y = pos_y[n / 2];
        let ret1 = pos_x.iter().map(|&x| (x - ret1_x).abs()).sum::<i64>()
            + pos_y.iter().map(|&y| (y - ret1_y).abs()).sum::<i64>();
        let ret2 = pos_x.iter().map(|&x| (x - ret2_x).abs()).sum::<i64>()
            + pos_y.iter().map(|&y| (y - ret2_y).abs()).sum::<i64>();

        writeln!(out, "{}", ret1.min(ret2)).unwrap();
    } else {
        let ret_x = pos_x[n / 2];
        let ret_y = pos_y[n / 2];
        let ret = pos_x.iter().map(|&x| (x - ret_x).abs()).sum::<i64>()
            + pos_y.iter().map(|&y| (y - ret_y).abs()).sum::<i64>();

        writeln!(out, "{ret}").unwrap();
    }
}
