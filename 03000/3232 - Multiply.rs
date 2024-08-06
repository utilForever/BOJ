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
        let (p, q, r) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let mut found = false;

        for b in 2..=16 {
            let p_converted = i64::from_str_radix(&p.to_string(), b);
            let q_converted = i64::from_str_radix(&q.to_string(), b);
            let r_converted = i64::from_str_radix(&r.to_string(), b);

            if p_converted.is_err() || q_converted.is_err() || r_converted.is_err() {
                continue;
            }

            let (p_converted, q_converted, r_converted) = (
                p_converted.unwrap(),
                q_converted.unwrap(),
                r_converted.unwrap(),
            );

            if p_converted * q_converted == r_converted {
                found = true;
                writeln!(out, "{b}").unwrap();
                break;
            }
        }

        if !found {
            writeln!(out, "0").unwrap();
        }
    }
}
