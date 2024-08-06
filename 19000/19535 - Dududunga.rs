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
    let mut points = vec![(0, 0); n - 1];
    let mut edges = vec![0_usize; n + 1];

    for i in 0..n - 1 {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        points[i] = (a, b);
        edges[a] += 1;
        edges[b] += 1;
    }

    let mut du = 0;
    let mut ga = 0;

    for i in 1..=n {
        if edges[i] >= 3 {
            let cnt = edges[i];
            ga += (cnt * (cnt - 1) * (cnt - 2)) / 6;
        }
    }

    while !points.is_empty() {
        let p = points.pop().unwrap();
        du += (edges[p.0] - 1) * (edges[p.1] - 1);
    }

    let ret = du as i64 - ga as i64 * 3;
    match ret {
        ret if ret > 0 => writeln!(out, "D").unwrap(),
        ret if ret < 0 => writeln!(out, "G").unwrap(),
        _ => writeln!(out, "DUDUDUNGA").unwrap(),
    }
}
