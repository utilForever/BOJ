use io::Write;
use std::{collections::BTreeMap, io, str};

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

    let n = scan.token::<usize>();
    let mut cars_start = BTreeMap::new();
    let mut cars_end = vec![String::new(); n];

    for i in 0..n {
        let car = scan.token::<String>();
        cars_start.insert(car.clone(), i);
    }

    for i in 0..n {
        cars_end[i] = scan.token::<String>();
    }

    let mut ret = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            // cars_end is the order of cars that leave the tunnel
            // cars_start is the order of cars that enter the tunnel
            // If cars_start.get(&cars_end[i]) > cars_start.get(&cars_end[j]),
            // it means that the car that leaves the tunnel first is the car that enters the tunnel later
            // So, we need to increase the ret value
            if cars_start.get(&cars_end[i]) > cars_start.get(&cars_end[j]) {
                ret += 1;
                break;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
