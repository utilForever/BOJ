use io::Write;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::f64::consts::PI;
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

#[derive(Copy, Clone, Debug)]
struct Event {
    z: f64,
    idx: usize,
    version: u64,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z && self.idx == other.idx && self.version == other.version
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.z.partial_cmp(&other.z) {
            Some(ord) => Some(ord.reverse()),
            None => None,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, h1, h2) = (
        scan.token::<usize>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );

    let mut vertices = vec![(0.0, 0.0); n];

    for i in 0..n {
        vertices[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    let mut degrees = vec![0.0; n];

    for i in 0..n {
        degrees[i] = scan.token::<f64>();
    }

    let mut nx = vec![0.0; n];
    let mut ny = vec![0.0; n];
    let mut tx = vec![0.0; n];
    let mut ty = vec![0.0; n];
    let mut d = vec![0.0; n];
    let mut speed = vec![0.0; n];
    let mut sin_a = vec![0.0; n];

    for i in 0..n {
        let j = (i + 1) % n;
        let ex = vertices[j].0 - vertices[i].0;
        let ey = vertices[j].1 - vertices[i].1;
        let len = (ex * ex + ey * ey).sqrt();

        tx[i] = ex / len;
        ty[i] = ey / len;

        nx[i] = -ey / len;
        ny[i] = ex / len;

        d[i] = nx[i] * vertices[i].0 + ny[i] * vertices[i].1;

        let rad = degrees[i] * PI / 180.0;
        sin_a[i] = rad.sin();
        speed[i] = 1.0 / rad.tan();
    }

    let mut prev: Vec<usize> = (0..n).map(|i| if i == 0 { n - 1 } else { i - 1 }).collect();
    let mut next: Vec<usize> = (0..n).map(|i| (i + 1) % n).collect();
    let mut alive = vec![true; n];
    let mut version = vec![0; n];

    let mut alpha = vec![0.0; n];
    let mut beta = vec![0.0; n];

    let proj_coeff = |j: usize,
                      k: usize,
                      target: usize,
                      nx: &Vec<f64>,
                      ny: &Vec<f64>,
                      d: &Vec<f64>,
                      speed: &Vec<f64>,
                      tx: &Vec<f64>,
                      ty: &Vec<f64>|
     -> (f64, f64) {
        let det = nx[j] * ny[k] - ny[j] * nx[k];

        if det.abs() < 1e-12 {
            return (0.0, 0.0);
        }

        let num1 = d[j] * ny[k] - d[k] * ny[j];
        let num2 = nx[j] * d[k] - nx[k] * d[j];

        let slope1 = speed[j] * ny[k] - speed[k] * ny[j];
        let slope2 = nx[j] * speed[k] - nx[k] * speed[j];

        let a = (tx[target] * num1 + ty[target] * num2) / det;
        let b = (tx[target] * slope1 + ty[target] * slope2) / det;

        (a, b)
    };

    let recompute = |i: usize,
                     prev: &Vec<usize>,
                     next: &Vec<usize>,
                     active: &Vec<bool>,
                     nx: &Vec<f64>,
                     ny: &Vec<f64>,
                     d: &Vec<f64>,
                     speed: &Vec<f64>,
                     tx: &Vec<f64>,
                     ty: &Vec<f64>,
                     alpha: &mut Vec<f64>,
                     beta: &mut Vec<f64>|
     -> f64 {
        if !active[i] {
            return f64::INFINITY;
        }

        let p = prev[i];
        let q = next[i];

        let (al, bl) = proj_coeff(p, i, i, nx, ny, d, speed, tx, ty);
        let (ar, br) = proj_coeff(i, q, i, nx, ny, d, speed, tx, ty);

        alpha[i] = ar - al;
        beta[i] = br - bl;

        if beta[i] < -1e-12 {
            -alpha[i] / beta[i]
        } else {
            f64::INFINITY
        }
    };

    let mut heap = BinaryHeap::new();
    let mut sum_alpha = 0.0;
    let mut sum_beta = 0.0;

    for i in 0..n {
        let zc = recompute(
            i, &prev, &next, &alive, &nx, &ny, &d, &speed, &tx, &ty, &mut alpha, &mut beta,
        );

        heap.push(Event {
            z: zc,
            idx: i,
            version: version[i],
        });

        sum_alpha += alpha[i] / sin_a[i];
        sum_beta += beta[i] / sin_a[i];
    }

    let mut last_z = 0.0f64;
    let mut area = 0.0;
    let mut cnt_alive = n;

    while let Some(Event {
        z: zc,
        idx: i,
        version: ver,
    }) = heap.pop()
    {
        if !alive[i] || ver != version[i] {
            continue;
        }

        if zc >= h2 {
            break;
        }

        let z_low = last_z.max(h1);
        let z_high = zc.min(h2);

        if z_high > z_low && cnt_alive >= 3 {
            area +=
                sum_alpha * (z_high - z_low) + 0.5 * sum_beta * (z_high * z_high - z_low * z_low);
        }

        last_z = zc;

        sum_alpha -= alpha[i] / sin_a[i];
        sum_beta -= beta[i] / sin_a[i];

        let p = prev[i];
        let q = next[i];

        next[p] = q;
        prev[q] = p;
        alive[i] = false;
        cnt_alive -= 1;

        if cnt_alive < 3 {
            sum_alpha = 0.0;
            sum_beta = 0.0;
            break;
        }

        for &j in [p, q].iter() {
            if !alive[j] {
                continue;
            }

            sum_alpha -= alpha[j] / sin_a[j];
            sum_beta -= beta[j] / sin_a[j];
            version[j] += 1;

            let z_new = recompute(
                j, &prev, &next, &alive, &nx, &ny, &d, &speed, &tx, &ty, &mut alpha, &mut beta,
            );

            heap.push(Event {
                z: z_new,
                idx: j,
                version: version[j],
            });

            sum_alpha += alpha[j] / sin_a[j];
            sum_beta += beta[j] / sin_a[j];
        }
    }

    let z_low = last_z.max(h1);
    let z_high = h2;

    if z_high > z_low && sum_alpha != 0.0 {
        area += sum_alpha * (z_high - z_low) + 0.5 * sum_beta * (z_high * z_high - z_low * z_low);
    }

    writeln!(out, "{:.12}", area).unwrap();
}
