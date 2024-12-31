use io::Write;
use std::collections::HashMap;
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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        points[i] = (x, y);
    }

    let mut ret = 0;

    for i in 0..n {
        let mut points_map = HashMap::new();

        for j in 0..n {
            let xxi = points[i].0 * points[i].0;
            let yyi = points[i].1 * points[i].1;
            let xxj = points[j].0 * points[j].0;
            let yyj = points[j].1 * points[j].1;

            let mut diff_x = xxi - xxj;
            let mut diff_y = yyj - yyi;

            if diff_x == 0 && diff_y == 0 {
                points_map
                    .entry((0, 0))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }

            if diff_x == 0 || diff_y == 0 {
                continue;
            }

            let g = gcd(diff_x, diff_y);

            diff_x /= g;
            diff_y /= g;

            if diff_y < 0 {
                diff_x *= -1;
                diff_y *= -1;
            }

            if diff_x < 0 {
                continue;
            }

            points_map
                .entry((diff_x, diff_y))
                .and_modify(|e| *e += 1)
                .or_insert(2);
        }

        for (_, v) in points_map.iter() {
            ret = ret.max(*v);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
