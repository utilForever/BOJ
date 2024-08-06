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
    let mut graph = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            graph[i][j] = scan.token::<usize>();
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if graph[i][k] == 1 && graph[k][j] == 1 {
                    graph[i][j] = 1;
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{} ", graph[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
