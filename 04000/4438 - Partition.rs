use io::Write;
use std::{cmp::Ordering, io, str};

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

    loop {
        let (n, w, h) = (
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if n == 0 && w == 0 && h == 0 {
            break;
        }

        let mut trees = vec![(0, 0, 0, 0); n];

        for i in 0..n {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            trees[i] = (x, y, 2 * x - w, 2 * y - h);
        }

        let mut order = (0..n).collect::<Vec<_>>();

        order.sort_unstable_by(|&i, &j| {
            let (a, b) = (&trees[i], &trees[j]);
            let is_a_upper = a.3 > 0 || (a.3 == 0 && a.2 > 0);
            let is_b_upper = b.3 > 0 || (b.3 == 0 && b.2 > 0);

            if is_a_upper != is_b_upper {
                return if is_a_upper {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }

            let cross = a.2 * b.3 - a.3 * b.2;

            if cross != 0 {
                return if cross > 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }

            let dir_a = a.2 * a.2 + a.3 * a.3;
            let dir_b = b.2 * b.2 + b.3 * b.3;

            dir_a.cmp(&dir_b)
        });

        let mut duplicate = Vec::with_capacity(2 * n);
        duplicate.extend_from_slice(&order);
        duplicate.extend_from_slice(&order);

        let mut pointer1 = 0;
        let mut start = 0;
        let mut dir_x = 1;
        let mut dir_y = 0;

        while start < n {
            let base = order[start];
            let mut end = start + 1;

            while end < n {
                let idx = order[end];
                let cross = trees[base].2 * trees[idx].3 - trees[base].3 * trees[idx].2;

                if cross == 0 {
                    end += 1;
                } else {
                    break;
                }
            }

            pointer1 = pointer1.max(end);

            while pointer1 < start + n {
                let idx = duplicate[pointer1];
                let cross = trees[base].2 * trees[idx].3 - trees[base].3 * trees[idx].2;

                if cross > 0 {
                    pointer1 += 1;
                } else {
                    break;
                }
            }

            let left = pointer1 - end;
            let mut pointer2 = pointer1;
            let mut cnt_opposite = 0;

            while pointer2 < start + n {
                let idx = duplicate[pointer2];
                let cross = trees[base].2 * trees[idx].3 - trees[base].3 * trees[idx].2;

                if cross != 0 {
                    break;
                }

                let dot = trees[base].2 * trees[idx].2 + trees[base].3 * trees[idx].3;

                if dot < 0 {
                    cnt_opposite += 1;
                    pointer2 += 1;
                } else {
                    break;
                }
            }

            let on_line = (end - start) + cnt_opposite;

            if left <= n / 2 && left + on_line >= n / 2 {
                dir_x = trees[base].2;
                dir_y = trees[base].3;
                break;
            }

            start = end;
        }

        let mut points_left = Vec::new();
        let mut points_on = Vec::new();

        for i in 0..n {
            let cross = dir_x * trees[i].3 - dir_y * trees[i].2;

            if cross > 0 {
                points_left.push(i);
            } else if cross == 0 {
                points_on.push(i);
            }
        }

        for idx in points_left.iter() {
            writeln!(out, "{} {}", trees[*idx].0, trees[*idx].1).unwrap();
        }

        for idx in points_on.iter().take(n / 2 - points_left.len()) {
            writeln!(out, "{} {}", trees[*idx].0, trees[*idx].1).unwrap();
        }
    }
}
