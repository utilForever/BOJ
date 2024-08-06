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

extern "C" {
    fn rand() -> u32;
}

fn get_dist(p1: &(i64, i64, usize, u32), p2: &(i64, i64, usize, u32)) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, d) = (scan.token::<usize>(), scan.token::<i64>());
    let mut sensors = vec![(0, 0, 0, 0); n];

    for i in 0..n {
        sensors[i] = (scan.token::<i64>(), scan.token::<i64>(), i, 0);
    }

    let mut ret = Vec::new();

    for _ in 0..1000 {
        sensors.iter_mut().for_each(|val| val.3 = unsafe { rand() });
        sensors.sort_by(|a, b| a.3.cmp(&b.3));

        let mut ret_local: Vec<(i64, i64, usize, u32)> = Vec::new();

        for sensor in sensors.iter() {
            if ret_local.iter().all(|val| get_dist(sensor, val) <= d * d) {
                ret_local.push(*sensor);
            }
        }

        if ret.len() < ret_local.len() {
            ret = ret_local;
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for idx in ret.iter() {
        write!(out, "{} ", idx.2 + 1).unwrap();
    }
    writeln!(out).unwrap();
}
