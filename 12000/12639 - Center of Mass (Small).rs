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

fn get_dist(position: (f64, f64, f64), velocity: (f64, f64, f64), t: f64) -> f64 {
    let x = position.0 + velocity.0 * t;
    let y = position.1 + velocity.1 * t;
    let z = position.2 + velocity.2 * t;

    (x * x + y * y + z * z).sqrt()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut position = (0.0, 0.0, 0.0);
        let mut velocity = (0.0, 0.0, 0.0);

        for _ in 0..n {
            position.0 += scan.token::<i64>() as f64;
            position.1 += scan.token::<i64>() as f64;
            position.2 += scan.token::<i64>() as f64;

            velocity.0 += scan.token::<i64>() as f64;
            velocity.1 += scan.token::<i64>() as f64;
            velocity.2 += scan.token::<i64>() as f64;
        }

        let a = velocity.0 * velocity.0 + velocity.1 * velocity.1 + velocity.2 * velocity.2;
        let b = position.0 * velocity.0 + position.1 * velocity.1 + position.2 * velocity.2;
        let t = 0.0_f64.max(-b / a);

        writeln!(
            out,
            "Case #{}: {:.10} {:.10}",
            i,
            get_dist(position, velocity, t) / n as f64,
            t
        )
        .unwrap();
    }
}
