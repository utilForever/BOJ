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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut ports = vec![(0, 0); n + 1];
    let mut directions = vec![' '; m];
    let mut visited = vec![(false, 0); n + 1];

    for i in 1..=n {
        ports[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    for i in 0..m {
        let direction = scan.token::<String>();
        directions[i] = direction.chars().next().unwrap();
    }

    let mut idx = 0;
    let mut idx_cur = 0;
    let mut idx_prev = 0;
    let mut ret = 1;

    while idx < k {
        for i in 0..m {
            if directions[i] == 'L' {
                ret = ports[ret].0;
            } else {
                ret = ports[ret].1;
            }
        }

        idx += 1;

        if visited[ret].0 {
            idx_cur = idx;
            idx_prev = visited[ret].1;
            break;
        }

        visited[ret].0 = true;
        visited[ret].1 = idx;
    }

    if idx != k {
        let k = k - idx;
        let remain = k % (idx_cur - idx_prev);

        for _ in 0..remain {
            for i in 0..m {
                if directions[i] == 'L' {
                    ret = ports[ret].0;
                } else {
                    ret = ports[ret].1;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
