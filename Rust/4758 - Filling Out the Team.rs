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

    loop {
        let (velocity, weight, strength) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        if velocity == 0.0 && weight == 0.0 && strength == 0.0 {
            break;
        }

        let mut matched = false;

        if velocity <= 4.5 && weight >= 150.0 && strength >= 200.0 {
            matched = true;
            write!(out, "Wide Receiver ").unwrap();
        }
        if velocity <= 6.0 && weight >= 300.0 && strength >= 500.0 {
            matched = true;
            write!(out, "Lineman ").unwrap();
        }
        if velocity <= 5.0 && weight >= 200.0 && strength >= 300.0 {
            matched = true;
            write!(out, "Quarterback").unwrap();
        }

        if !matched {
            write!(out, "No positions").unwrap();
        }

        writeln!(out).unwrap();
    }
}
