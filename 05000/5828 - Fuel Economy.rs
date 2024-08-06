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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, g, b, d) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut gas_stations = vec![(0, 0); n + 1];

    for i in 1..=n {
        gas_stations[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    gas_stations.push((d, 0));
    gas_stations.sort_by(|a, b| a.0.cmp(&b.0));

    let mut stack: Vec<(i64, i64)> = Vec::new();
    let mut next_station = vec![0; n + 2];

    for i in (1..=n + 1).rev() {
        while !stack.is_empty() && stack.last().unwrap().0 >= gas_stations[i].1 {
            stack.pop();
        }

        next_station[i] = if stack.is_empty() {
            -1
        } else {
            stack.last().unwrap().1
        };
        stack.push((gas_stations[i].1, i as i64));
    }

    let mut fuel_remain = b;
    let mut ret = 0;

    for i in 1..=n + 1 {
        fuel_remain -= gas_stations[i].0 - gas_stations[i - 1].0;

        if fuel_remain < 0 {
            writeln!(out, "-1").unwrap();
            return;
        }

        let fuel_needed = if next_station[i] == -1 {
            (d - gas_stations[i].0).min(g)
        } else {
            let dist = gas_stations[next_station[i] as usize].0 - gas_stations[i].0;
            (dist.min(g) - fuel_remain).max(0)
        };

        ret += fuel_needed * gas_stations[i].1;
        fuel_remain += fuel_needed;
    }

    writeln!(out, "{ret}").unwrap();
}
