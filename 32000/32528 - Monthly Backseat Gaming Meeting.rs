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
    let mut graph = vec![0; n + 1];
    let (mut a, mut b) = (scan.token::<usize>(), scan.token::<usize>());

    for i in 1..=n {
        graph[i] = scan.token::<usize>();
    }

    if a == b {
        if (n - 1) % 2 == 1 {
            writeln!(out, "first").unwrap();
            return;
        }

        let mut is_first = true;

        for _ in 0..2 * n {
            if is_first && graph[a] != b {
                writeln!(out, "second").unwrap();
                return;
            } else if !is_first && graph[b] != a {
                writeln!(out, "first").unwrap();
                return;
            }

            if is_first {
                a = graph[a];
            } else {
                b = graph[b];
            }

            is_first ^= true;
        }

        writeln!(out, "draw").unwrap();
    } else {
        if (n - 2) % 2 == 1 {
            writeln!(out, "first").unwrap();
            return;
        }

        let mut is_first = true;

        for _ in 0..2 * n {
            if is_first && graph[a] == b {
                writeln!(out, "second").unwrap();
                return;
            } else if !is_first && graph[b] == a {
                writeln!(out, "first").unwrap();
                return;
            }

            if is_first {
                a = graph[a];
            } else {
                b = graph[b];
            }

            is_first ^= true;
        }

        writeln!(out, "draw").unwrap();
    }
}
