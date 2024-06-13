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

    let n = scan.token::<usize>();
    let mut passengers = vec![0; n + 1];

    for i in 1..=n {
        passengers[i] = scan.token::<usize>();
    }

    let len_airplane = *passengers.iter().max().unwrap();
    let mut airplane = vec![0; len_airplane + 1];

    for i in 1..=n {
        // For each passenger, we calculate the time needed to reach his seat
        // Case 1: the passenger moves to the right immediately
        // Case 2: the passenger waits for the passenger on the right that waits for the second passenger on the right (chaining)
        // Case 3: the passenger waits for the passenger on the right that puts his luggage
        for j in 1..passengers[i] {
            airplane[j] = (airplane[j - 1] + 1)
                .max(airplane[j] + 1)
                .max(airplane[j + 1]);
        }

        airplane[passengers[i]] = airplane[passengers[i]].max(airplane[passengers[i] - 1]) + 5;
    }

    writeln!(out, "{}", airplane.iter().max().unwrap()).unwrap();
}
