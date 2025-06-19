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

const EPS: f64 = 1e-7;

#[derive(Default, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dist(self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn triplet_min_sum(p: Point, a: Point, b: Point) -> f64 {
    let ap = p.dist(a);
    let bp = p.dist(b);
    let ab = a.dist(b);

    let dot_p = (a.x - p.x) * (b.x - p.x) + (a.y - p.y) * (b.y - p.y);

    if dot_p <= -0.5 * ap * bp {
        return ap + bp;
    }

    let dot_a = (p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y);

    if dot_a <= -0.5 * ap * ab {
        return ap + ab;
    }

    let dot_b = (p.x - b.x) * (a.x - b.x) + (p.y - b.y) * (a.y - b.y);

    if dot_b <= -0.5 * bp * ab {
        return bp + ab;
    }

    let area = 0.5 * ((a.x - p.x) * (b.y - p.y) - (a.y - p.y) * (b.x - p.x)).abs();

    ((ap * ap + bp * bp + ab * ab + 4.0 * 3.0f64.sqrt() * area) / 2.0).sqrt()
}

fn calculate_max(p: Point, friends: &[Point; 3]) -> f64 {
    let sum1 = triplet_min_sum(p, friends[0], friends[1]);
    let sum2 = triplet_min_sum(p, friends[0], friends[2]);
    let sum3 = triplet_min_sum(p, friends[1], friends[2]);

    sum1.max(sum2).max(sum3)
}

fn gradient(p: Point, friends: &[Point; 3]) -> (f64, f64) {
    let val_x = (calculate_max(Point::new(p.x + EPS, p.y), friends)
        - calculate_max(Point::new(p.x - EPS, p.y), friends))
        / (2.0 * EPS);
    let val_y = (calculate_max(Point::new(p.x, p.y + EPS), friends)
        - calculate_max(Point::new(p.x, p.y - EPS), friends))
        / (2.0 * EPS);

    (val_x, val_y)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut friends = [Point::default(); 3];

    for i in 0..3 {
        friends[i] = Point::new(scan.token::<f64>(), scan.token::<f64>());
    }

    let mut point = Point::new(
        (friends[0].x + friends[1].x + friends[2].x) / 3.0,
        (friends[0].y + friends[1].y + friends[2].y) / 3.0,
    );
    let (mut mean_x, mut mean_y) = (0.0, 0.0);
    let (mut variance_x, mut variance_y) = (0.0, 0.0);
    let mut learning_rate = 100.0f64;
    let (beta1, beta2) = (0.9f64, 0.999f64);

    for t in 1..=10000 {
        let (grad_x, grad_y) = gradient(point, &friends);

        mean_x = beta1 * mean_x + (1.0 - beta1) * grad_x;
        mean_y = beta1 * mean_y + (1.0 - beta1) * grad_y;
        variance_x = beta2 * variance_x + (1.0 - beta2) * grad_x * grad_x;
        variance_y = beta2 * variance_y + (1.0 - beta2) * grad_y * grad_y;

        let m_hat_x = mean_x / (1.0 - beta1.powi(t));
        let m_hat_y = mean_y / (1.0 - beta1.powi(t));
        let v_hat_x = variance_x / (1.0 - beta2.powi(t));
        let v_hat_y = variance_y / (1.0 - beta2.powi(t));

        point.x -= learning_rate * m_hat_x / (v_hat_x.sqrt() + EPS);
        point.y -= learning_rate * m_hat_y / (v_hat_y.sqrt() + EPS);

        learning_rate *= 0.995;
    }

    let ret = calculate_max(point, &friends);
    writeln!(out, "{:.12}", ret).unwrap();
}
