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

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut ants = vec![(0.0, 0.0); n];

        for j in 0..n {
            ants[j] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let (min_x, max_x, min_y, max_y) = ants.iter().fold(
            (ants[0].0, ants[0].0, ants[0].1, ants[0].1),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
            },
        );

        writeln!(
            out,
            "Case {i}: Area {:.10}, Perimeter {:.10}",
            (max_x - min_x) * (max_y - min_y),
            2.0 * (max_x - min_x + max_y - min_y)
        )
        .unwrap();
    }
}
