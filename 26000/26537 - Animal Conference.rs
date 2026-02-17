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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    for _ in 0..n {
        let z = scan.token::<usize>();
        let mut coords = vec![(0, 0); z];

        for i in 0..z {
            coords[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        let mut dist_min = f64::MAX;
        let (mut x_min, mut y_min) = (i64::MAX, i64::MAX);
        let (mut x_max, mut y_max) = (i64::MIN, i64::MIN);

        for i in 0..z {
            for j in i + 1..z {
                let dist = ((coords[i].0 - coords[j].0).pow(2) + (coords[i].1 - coords[j].1).pow(2))
                    as f64;

                if dist < dist_min
                    || (dist == dist_min
                        && (x_min, y_min, x_max, y_max)
                            > (
                                coords[i].0.min(coords[j].0),
                                coords[i].1.min(coords[j].1),
                                coords[i].0.max(coords[j].0),
                                coords[i].1.max(coords[j].1),
                            ))
                {
                    dist_min = dist;
                    x_min = coords[i].0.min(coords[j].0);
                    y_min = coords[i].1.min(coords[j].1);
                    x_max = coords[i].0.max(coords[j].0);
                    y_max = coords[i].1.max(coords[j].1);
                }
            }
        }

        writeln!(out, "{x_min} {y_min} {x_max} {y_max}").unwrap();
    }
}
