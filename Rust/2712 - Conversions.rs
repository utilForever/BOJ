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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (value, unit) = (scan.token::<f64>(), scan.token::<String>());
        let ret = match unit.as_str() {
            "kg" => value * 2.2046,
            "lb" => value * 0.4536,
            "l" => value * 0.2642,
            "g" => value * 3.7854,
            _ => unreachable!(),
        };

        writeln!(
            out,
            "{:.4} {}",
            ret,
            match unit.as_str() {
                "kg" => "lb",
                "lb" => "kg",
                "l" => "g",
                "g" => "l",
                _ => unreachable!(),
            }
        )
        .unwrap();
    }
}
