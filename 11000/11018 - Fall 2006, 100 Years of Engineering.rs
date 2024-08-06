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

#[derive(Default, Clone)]
struct Rocket {
    weight: f64,
    time: f64,
    force: f64,
}

impl Rocket {
    fn new(weight: f64, time: f64, force: f64) -> Self {
        Self {
            weight,
            time,
            force,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (n, m) = (scan.token::<usize>(), scan.token::<f64>());
        let mut rockets = vec![Rocket::default(); n];
        let mut weight = m;
        let mut velocity = 0.0;
        let mut height = 0.0;

        for j in 0..n {
            rockets[j] = Rocket::new(
                scan.token::<f64>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );
            weight += rockets[j].weight;
        }

        for rocket in rockets {
            let acceleration = rocket.force / weight - 9.81;
            height += velocity * rocket.time + 0.5 * acceleration * rocket.time.powi(2);
            velocity += acceleration * rocket.time;
            weight -= rocket.weight;
        }

        writeln!(out, "Data Set {i}:").unwrap();
        writeln!(out, "{:.2}", height).unwrap();

        if i != k {
            writeln!(out).unwrap();
        }
    }
}
