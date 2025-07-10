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

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

fn split(points: &mut Vec<Point>, axis: Axis) -> i64 {
    if axis == Axis::X {
        points.sort_unstable_by_key(|p| p.x);
    } else {
        points.sort_unstable_by_key(|p| p.y);
    }

    let n = points.len();
    let mut prefix_x_min = vec![0; n];
    let mut prefix_x_max = vec![0; n];
    let mut prefix_y_min = vec![0; n];
    let mut prefix_y_max = vec![0; n];

    prefix_x_min[0] = points[0].x;
    prefix_x_max[0] = points[0].x;
    prefix_y_min[0] = points[0].y;
    prefix_y_max[0] = points[0].y;

    for i in 1..n {
        prefix_x_min[i] = prefix_x_min[i - 1].min(points[i].x);
        prefix_x_max[i] = prefix_x_max[i - 1].max(points[i].x);
        prefix_y_min[i] = prefix_y_min[i - 1].min(points[i].y);
        prefix_y_max[i] = prefix_y_max[i - 1].max(points[i].y);
    }

    let mut suffix_x_min = vec![0; n];
    let mut suffix_x_max = vec![0; n];
    let mut suffix_y_min = vec![0; n];
    let mut suffix_y_max = vec![0; n];

    suffix_x_min[n - 1] = points[n - 1].x;
    suffix_x_max[n - 1] = points[n - 1].x;
    suffix_y_min[n - 1] = points[n - 1].y;
    suffix_y_max[n - 1] = points[n - 1].y;

    for i in (0..n - 1).rev() {
        suffix_x_min[i] = suffix_x_min[i + 1].min(points[i].x);
        suffix_x_max[i] = suffix_x_max[i + 1].max(points[i].x);
        suffix_y_min[i] = suffix_y_min[i + 1].min(points[i].y);
        suffix_y_max[i] = suffix_y_max[i + 1].max(points[i].y);
    }

    let mut ret = i64::MAX;

    for i in 0..n - 1 {
        if (axis == Axis::X && points[i].x == points[i + 1].x)
            || (axis == Axis::Y && points[i].y == points[i + 1].y)
        {
            continue;
        }

        let area_left = (prefix_x_max[i] - prefix_x_min[i]) * (prefix_y_max[i] - prefix_y_min[i]);
        let area_right = (suffix_x_max[i + 1] - suffix_x_min[i + 1])
            * (suffix_y_max[i + 1] - suffix_y_min[i + 1]);
        ret = ret.min(area_left + area_right);
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut points = Vec::with_capacity(n);

    for _ in 0..n {
        points.push(Point::new(scan.token::<i64>(), scan.token::<i64>()));
    }

    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    let mut y_min = i64::MAX;
    let mut y_max = i64::MIN;

    for i in 0..n {
        x_min = x_min.min(points[i].x);
        x_max = x_max.max(points[i].x);
        y_min = y_min.min(points[i].y);
        y_max = y_max.max(points[i].y);
    }

    let area = (x_max - x_min) * (y_max - y_min);
    let mut fence_min = i64::MAX;

    fence_min = fence_min.min(split(&mut points.clone(), Axis::X));
    fence_min = fence_min.min(split(&mut points.clone(), Axis::Y));

    writeln!(out, "{}", area - fence_min).unwrap();
}
