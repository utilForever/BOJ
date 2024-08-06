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
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut points = vec![(0.0, 0.0); n];

        for i in 0..n {
            points[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let mut ret = 1;

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let (dx, dy) = (points[i].0 - points[j].0, points[i].1 - points[j].1);
                let dist_points = (dx * dx + dy * dy).sqrt();

                // Can't exceed 2.0 because the radius of the circle is 1.0
                if dist_points > 2.0 + 1e-6 {
                    continue;
                }

                // Calculate the height of the triangle by using the Pythagorean theorem
                let len_height = (1.0 - dist_points * dist_points / 4.0).sqrt();
                // Calculate normalized vector to the center of the circle
                let (vec_x, vec_y) = (
                    -dy / dist_points * len_height,
                    dx / dist_points * len_height,
                );
                // Calculate the center of the circle
                let (mid_x, mid_y) = (
                    (points[i].0 + points[j].0) / 2.0 + vec_x,
                    (points[i].1 + points[j].1) / 2.0 + vec_y,
                );

                let mut cnt = 0;

                for k in 0..n {
                    let len = (mid_x - points[k].0).powi(2) + (mid_y - points[k].1).powi(2);

                    if len < 1.0 + 1e-6 {
                        cnt += 1;
                    }
                }

                ret = ret.max(cnt);
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
