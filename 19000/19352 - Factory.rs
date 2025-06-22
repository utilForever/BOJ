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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn calculate_dist(points: &Vec<(i64, i64)>, pos_hub: (f64, f64)) -> f64 {
    points.iter().fold(0.0, |acc, &(x, y)| {
        acc + ((x as f64 - pos_hub.0).powi(2) + (y as f64 - pos_hub.1).powi(2)).sqrt()
    })
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let z = scan.token::<i64>();

    for _ in 0..z {
        let n = scan.token::<usize>();
        let mut points = vec![(0, 0); n];

        for i in 0..n {
            points[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        let mut pos_store = (0.0, 0.0);
        let mut delta = 500000.0;

        while delta > 1e-6 {
            let mut pos_best = pos_store;
            let mut dist_best = calculate_dist(&points, pos_store);

            for dx in [-delta, 0.0, delta].iter() {
                for dy in [-delta, 0.0, delta].iter() {
                    let pos_new = (pos_store.0 + dx, pos_store.1 + dy);
                    let dist_new = calculate_dist(&points, pos_new);

                    if dist_new < dist_best {
                        pos_best = pos_new;
                        dist_best = dist_new;
                    }
                }
            }

            pos_store = pos_best;
            delta *= 0.995;
        }

        writeln!(out, "{:.6} {:.6}", pos_store.0, pos_store.1).unwrap();
    }
}
