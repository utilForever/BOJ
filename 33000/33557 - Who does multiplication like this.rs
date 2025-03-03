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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
        let multiply_real = a * b;

        let (a, b) = (
            a.to_string().chars().collect::<Vec<_>>(),
            b.to_string().chars().collect::<Vec<_>>(),
        );
        let mut multiply_fake = String::new();

        if a.len() == b.len() {
            for i in 0..a.len() {
                let val_a = a[i].to_digit(10).unwrap();
                let val_b = b[i].to_digit(10).unwrap();
                multiply_fake.push_str(&format!("{}", val_a * val_b));
            }
        } else if a.len() > b.len() {
            for i in 0..a.len() - b.len() {
                multiply_fake.push_str(&format!("{}", a[i].to_digit(10).unwrap()));
            }

            for i in 0..b.len() {
                let val_a = a[i + a.len() - b.len()].to_digit(10).unwrap();
                let val_b = b[i].to_digit(10).unwrap();
                multiply_fake.push_str(&format!("{}", val_a * val_b));
            }
        } else {
            for i in 0..b.len() - a.len() {
                multiply_fake.push_str(&format!("{}", b[i].to_digit(10).unwrap()));
            }

            for i in 0..a.len() {
                let val_a = a[i].to_digit(10).unwrap();
                let val_b = b[i + b.len() - a.len()].to_digit(10).unwrap();
                multiply_fake.push_str(&format!("{}", val_a * val_b));
            }
        }

        let multiply_fake = multiply_fake.parse::<i64>().unwrap();

        writeln!(
            out,
            "{}",
            if multiply_real == multiply_fake {
                "1"
            } else {
                "0"
            }
        )
        .unwrap();
    }
}
