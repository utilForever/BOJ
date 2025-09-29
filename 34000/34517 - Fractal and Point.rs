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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b, l, k, p, q) = (
            scan.token::<i64>() * 1000,
            scan.token::<i64>() * 1000,
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        let p = p.split('.').collect::<Vec<&str>>();
        let q = q.split('.').collect::<Vec<&str>>();
        let sign_p = if p[0].starts_with('-') { -1 } else { 1 };
        let sign_q = if q[0].starts_with('-') { -1 } else { 1 };

        let p = p[0].parse::<i64>().unwrap() * 1000 + sign_p * p[1].parse::<i64>().unwrap() - a;
        let q = q[0].parse::<i64>().unwrap() * 1000 + sign_q * q[1].parse::<i64>().unwrap() - b;

        let len = 3i64.pow(k as u32) * 1000;

        if p < 0 || p > len || q < 0 || q > len {
            writeln!(out, "0").unwrap();
            continue;
        }

        if l == 0 {
            writeln!(out, "1").unwrap();
            continue;
        }

        let mut size = len / 3;
        let mut check = true;

        for _ in 0..l {
            let (x, y) = (p % (size * 3), q % (size * 3));

            if x > size && x < 2 * size && y > size && y < 2 * size {
                check = false;
                break;
            }

            size /= 3;
        }

        writeln!(out, "{}", if check { 1 } else { 0 }).unwrap();
    }
}
