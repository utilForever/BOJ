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

    let mut points = [(0, 0); 4];

    for i in 0..4 {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    points.sort();

    let cond_x =
        points[0].0 == points[1].0 && points[1].0 != points[2].0 && points[2].0 == points[3].0;
    let cond_y =
        points[0].1 != points[1].1 && points[0].1 == points[2].1 && points[1].1 == points[3].1;
    let cond_len = (points[0].1 - points[1].1).abs() == (points[0].0 - points[2].0).abs();

    writeln!(
        out,
        "{}",
        if cond_x && cond_y && cond_len {
            "TAK"
        } else {
            "NIE"
        }
    )
    .unwrap();
}
