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

    let mut idx = 1;

    loop {
        let n = scan.token::<i64>();

        if n == 0 {
            break;
        }

        let mut graph = vec![
            ("Jan", 0),
            ("Feb", 0),
            ("Mar", 0),
            ("Apr", 0),
            ("May", 0),
            ("Jun", 0),
            ("Jul", 0),
            ("Aug", 0),
            ("Sep", 0),
            ("Oct", 0),
            ("Nov", 0),
            ("Dec", 0),
        ];

        for _ in 0..n {
            let (_, m, _) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );

            graph[m as usize - 1].1 += 1;
        }

        writeln!(out, "Case #{idx}:").unwrap();

        for (month, count) in graph {
            write!(out, "{month}:").unwrap();

            for _ in 0..count {
                write!(out, "*").unwrap();
            }

            writeln!(out).unwrap();
        }

        idx += 1;
    }
}
