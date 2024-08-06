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

    let (width, height) = (scan.token::<i64>(), scan.token::<i64>());
    let n = scan.token::<i64>();
    let mut cut_width = vec![0, width];
    let mut cut_height = vec![0, height];

    for _ in 0..n {
        let (index, cut) = (scan.token::<i64>(), scan.token::<i64>());

        if index == 0 {
            cut_height.push(cut);
        } else {
            cut_width.push(cut);
        }
    }

    cut_width.sort();
    cut_height.sort();

    let mut ret = (0, 0);

    cut_width
        .windows(2)
        .for_each(|w| ret.0 = ret.0.max(w[1] - w[0]));
    cut_height
        .windows(2)
        .for_each(|w| ret.1 = ret.1.max(w[1] - w[0]));

    writeln!(out, "{}", ret.0 * ret.1).unwrap();
}
