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
        let (logic, w1, w2, b) = (
            scan.token::<String>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        let mut ret = Vec::new();

        for i in [0.0, 1.0] {
            for j in [0.0, 1.0] {
                let val = w1 * i + w2 * j + b;

                if val >= 0.0 {
                    ret.push(1);
                } else {
                    ret.push(0);
                }
            }
        }

        if logic == "AND" {
            writeln!(
                out,
                "{}",
                if ret == [0, 0, 0, 1] { "true" } else { "false" }
            )
            .unwrap();
        } else {
            writeln!(
                out,
                "{}",
                if ret == [0, 1, 1, 1] { "true" } else { "false" }
            )
            .unwrap();
        }
    }
}
