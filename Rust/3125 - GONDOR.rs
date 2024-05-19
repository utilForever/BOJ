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
    let mut sparks = vec![(0.0, 0.0, 0, Vec::new()); n + 1];

    for i in 1..=n {
        let (x, y, s) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<usize>(),
        );
        sparks[i] = (x, y, s, Vec::new());

        for _ in 0..n - 1 {
            let idx = scan.token::<usize>();
            sparks[i].3.push(idx);
        }
    }

    let mut visited = vec![false; n + 1];
    let mut ret = vec![f64::MAX / 4.0; n + 1];

    ret[1] = 0.0;

    for _ in 1..=n {
        let mut curr = 0;

        for j in 1..=n {
            if visited[j] {
                continue;
            }

            if ret[j] < ret[curr] {
                curr = j;
            }
        }

        visited[curr] = true;

        let mut dists = vec![f64::MAX / 4.0; n + 1];
        let mut cnt = 0;

        for j in 1..=n {
            dists[j] = ((sparks[curr].0 - sparks[j].0).powi(2)
                + (sparks[curr].1 - sparks[j].1).powi(2))
            .sqrt();
        }

        for &next in sparks[curr].3.iter() {
            if visited[next] {
                continue;
            }

            ret[next] = ret[next].min(ret[curr] + dists[next]);
            cnt += 1;

            if cnt == sparks[curr].2 {
                break;
            }
        }
    }

    for i in 1..=n {
        writeln!(out, "{:.10}", ret[i]).unwrap();
    }
}
