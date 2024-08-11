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
        let (mut odometer, mut fuel) = (scan.token::<f64>(), scan.token::<f64>());

        if odometer == -1.0 && fuel == -1.0 {
            break;
        }

        let mut total_dist = 0.0;
        let mut total_fuel = 0.0;

        loop {
            let (odometer_next, fuel_next) = (scan.token::<f64>(), scan.token::<f64>());

            if odometer_next == 0.0 && fuel_next == 0.0 {
                break;
            }

            if fuel < fuel_next {
                odometer = odometer_next;
                fuel = fuel_next;
                continue;
            }

            total_dist += odometer_next - odometer;
            total_fuel += fuel - fuel_next;

            odometer = odometer_next;
            fuel = fuel_next;
        }

        let ratio = total_dist / total_fuel;

        writeln!(out, "{}", (fuel * ratio).round() as i64).unwrap();
    }
}
