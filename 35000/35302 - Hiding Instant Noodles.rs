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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (_, k) = (scan.token::<i64>(), scan.token::<usize>());
    let mut locations = vec![(0, 0, 0); k];

    for i in 0..k {
        locations[i] = (scan.token::<i64>(), scan.token::<i64>(), i);
    }

    locations.sort_unstable_by(|a, b| {
        let lhs = ((a.0 + 2) / 2) * b.1;
        let rhs = a.1 * ((b.0 + 2) / 2);

        if lhs == rhs {
            a.2.cmp(&b.2)
        } else {
            lhs.cmp(&rhs)
        }
    });

    let mut prefix_sum = 0;
    let mut ret = 0;

    for location in locations.iter() {
        prefix_sum += (location.0 + 2) / 2;
        ret += prefix_sum * location.1;
    }

    let val_location_even = locations
        .iter()
        .filter(|location| location.0 % 2 == 0)
        .map(|location| location.1)
        .sum::<i64>();

    writeln!(out, "{}", 2 * ret - val_location_even).unwrap();
}
