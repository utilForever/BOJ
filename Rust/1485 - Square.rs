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
        let mut points = [(0, 0); 4];

        for i in 0..4 {
            points[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        let mut dists = Vec::new();

        for i in 0..3 {
            for j in i + 1..4 {
                let dist = (points[j].0 - points[i].0).pow(2) + (points[j].1 - points[i].1).pow(2);
                dists.push(dist);
            }
        }

        dists.sort();

        writeln!(
            out,
            "{}",
            if dists[0] == dists[1]
                && dists[1] == dists[2]
                && dists[2] == dists[3]
                && dists[4] == dists[5]
            {
                "1"
            } else {
                "0"
            }
        )
        .unwrap();
    }
}
