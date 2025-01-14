use io::Write;
use std::{
    io::{self, BufWriter, StdoutLock},
    str,
};

type Out<'a> = BufWriter<StdoutLock<'a>>;

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

fn calculate_dist(
    points_blue: &[(f64, f64)],
    points_yellow: &[(f64, f64)],
    x: f64,
    y: f64,
    out: &mut Out,
    found: &mut bool,
) -> f64 {
    let mut diff_yellow_min = f64::MAX;
    let mut diff_blue_max = f64::MIN;

    // Calculate the maximum distance between the blue points
    for i in 0..points_blue.len() {
        // x^2 + y^2 = r^2
        // dist^2 = (x-px)^2 + (y-py)^2
        //        = x^2 - 2*px*x + px^2 + y^2 - 2*py*y + py^2
        // r^2 - dist^2 = (x^2 + y^2) - (x^2 - 2*px*x + px^2 + y^2 - 2*py*y + py^2)
        //              = 2*px*x + 2*py*y - px^2 - py^2
        let diff = 2.0 * points_blue[i].0 * x + 2.0 * points_blue[i].1 * y
            - points_blue[i].0.powi(2)
            - points_blue[i].1.powi(2);

        if diff > diff_blue_max {
            diff_blue_max = diff;
        }
    }

    // Calculate the minimum distance between the yellow points
    for i in 0..points_yellow.len() {
        let diff = 2.0 * points_yellow[i].0 * x + 2.0 * points_yellow[i].1 * y
            - points_yellow[i].0.powi(2)
            - points_yellow[i].1.powi(2);

        if diff < diff_yellow_min {
            diff_yellow_min = diff;
        }
    }

    // If we discover a point where diff_yellow_min > diff_blue_max, we found the answer.
    // In other words, we can draw a circle that separates the blue and yellow points
    // such that the blue points are inside the circle and the yellow points are outside the circle.
    if diff_yellow_min > diff_blue_max && !*found {
        *found = true;

        writeln!(
            out,
            "{}",
            (x * x + y * y - (diff_blue_max + diff_yellow_min) / 2.0).sqrt()
        )
        .unwrap();
        writeln!(out, "{x} {y}").unwrap();
    }

    diff_yellow_min - diff_blue_max
}

// Performs ternary search on the y-coordinate for a fixed x-coordinate,
// looking to optimize the distance measure defined by 'calculate_dist'.
fn process_ternary_search_y(
    points_blue: &[(f64, f64)],
    points_yellow: &[(f64, f64)],
    x: f64,
    out: &mut Out,
    found: &mut bool,
) -> f64 {
    let mut left_y = -1_000_000_000.0;
    let mut right_y = 1_000_000_000.0;

    for _ in 0..60 {
        let mid1_y = left_y + (right_y - left_y) / 3.0;
        let mid2_y = right_y - (right_y - left_y) / 3.0;

        let ret1 = calculate_dist(points_blue, points_yellow, x, mid1_y, out, found);
        let ret2 = calculate_dist(points_blue, points_yellow, x, mid2_y, out, found);

        if *found {
            return 0.0;
        }

        if ret1 > ret2 {
            right_y = mid2_y;
        } else {
            left_y = mid1_y;
        }
    }

    return calculate_dist(points_blue, points_yellow, x, left_y, out, found);
}

// Performs a ternary search on the x-coordinate, and for each candidate x,
// calls process_ternary_search_y to find the best y.
fn process_ternary_search_x(
    points_blue: &[(f64, f64)],
    points_yellow: &[(f64, f64)],
    out: &mut Out,
    found: &mut bool,
) {
    let mut left_x = -1_000_000_000.0;
    let mut right_x = 1_000_000_000.0;

    for _ in 0..60 {
        let mid1_x = left_x + (right_x - left_x) / 3.0;
        let mid2_x = right_x - (right_x - left_x) / 3.0;

        let ret1 = process_ternary_search_y(points_blue, points_yellow, mid1_x, out, found);
        let ret2 = process_ternary_search_y(points_blue, points_yellow, mid2_x, out, found);

        if *found {
            return;
        }

        if ret1 > ret2 {
            right_x = mid2_x;
        } else {
            left_x = mid1_x;
        }
    }
}

// Reference: Petrozavodsk Winter 2022. Day 5. Yandex Cup Editorial
// Reference: https://codeforces.com/blog/entry/61710
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut points_blue = vec![(0.0, 0.0); n];

        for i in 0..n {
            points_blue[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let m = scan.token::<usize>();
        let mut points_yellow = vec![(0.0, 0.0); m];

        for i in 0..m {
            points_yellow[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let mut found = false;

        // Perform ternary search in the x dimension (and nested y dimension).
        process_ternary_search_x(&points_blue, &points_yellow, &mut out, &mut found);
    }
}
