use io::Write;
use std::{io, str};

static MIN_Y_BOUND: f32 = -13.0;
static MAX_Y_BOUND: f32 = 13.0;
static START_X: f32 = -10.0;
static END_X: f32 = 10.0;
static MIN_A: f32 = -10.0;
static MAX_A: f32 = 10.0;
static MIN_C: f32 = -10.0;
static MAX_C: f32 = 10.0;
static X_COORD: f32 = 0.0;
static MIN_SLOPE: f32 = (MIN_C - MAX_A) / (X_COORD - START_X);
static MAX_SLOPE: f32 = (MAX_C - MIN_A) / (X_COORD - START_X);
static H: f32 = 0.01;

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

fn process_binary_search(a: f32, b: f32, islands: &Vec<f32>, mut left: f32, mut right: f32) -> f32 {
    let mut dose = f32::MAX;

    while (right - left).abs() / 2.0 > f32::EPSILON {
        let mid = (left + right) / 2.0;

        let result = calculate_dose(islands, START_X, a, mid);

        dose = result.0;
        let y = result.1;

        if y < b {
            left = mid;
        } else {
            right = mid;
        }
    }

    dose
}

fn calculate_dose(islands: &Vec<f32>, mut x: f32, mut y: f32, mut yp: f32) -> (f32, f32) {
    let mut dose = 0.0;

    for _ in 0..((END_X - START_X) / H) as i32 {
        if y < MIN_Y_BOUND || y > MAX_Y_BOUND {
            return (f32::MAX, y);
        }

        dose += H * (1.0 + get_determinant(islands, x, y)) * (1.0 + yp * yp).sqrt();

        let k1 = H * yp;
        let l1 = H * calculate_euler_lagrange(islands, x, y, yp);

        x += H;
        y += k1;
        yp += l1;
    }

    (dose, y)
}

fn get_determinant(islands: &Vec<f32>, x: f32, y: f32) -> f32 {
    let mut dose = 0.0;

    for island in islands.iter() {
        let d_square = x * x + (y - island) * (y - island);

        if d_square < f32::EPSILON {
            return f32::MAX;
        }

        dose += 1.0 / d_square;
    }

    dose
}

fn calculate_euler_lagrange(islands: &Vec<f32>, x: f32, y: f32, yp: f32) -> f32 {
    let t = 1.0 + yp * yp;
    let mut s = 1.0;
    let mut syp = 0.0;
    let mut sx = 0.0;

    for island in islands.iter() {
        let d_square = x * x + (y - island) * (y - island);

        s += 1.0 / d_square;
        syp += (y - island) / (d_square * d_square);
        sx += (x + (y - island) * yp) / (d_square * d_square);
    }

    2.0 * t * (sx * yp - syp * t) / s
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let (n, a, b) = (
            scan.token::<usize>(),
            scan.token::<f32>(),
            scan.token::<f32>(),
        );
        let mut islands = vec![0.0; n];

        for j in 0..n {
            islands[j] = scan.token::<f32>();
        }

        let mut slopes = vec![MIN_SLOPE, MAX_SLOPE];

        for island in islands.iter() {
            slopes.push((island - a) / (X_COORD - START_X));
        }

        slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut min_radiation_dose = f32::MAX;

        for j in 0..slopes.len() - 1 {
            let result = process_binary_search(a, b, &islands, slopes[j], slopes[j + 1]);

            if min_radiation_dose > result {
                min_radiation_dose = result;
            }
        }

        writeln!(out, "Case #{i}: {min_radiation_dose}").unwrap();
    }
}
