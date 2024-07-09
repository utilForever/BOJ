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

    let (n, d, k, c) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut sushis = vec![0; n];

    for i in 0..n {
        sushis[i] = scan.token::<usize>();
    }

    sushis.extend(sushis.clone().iter().take(k - 1));

    let mut ret = 0;

    sushis.windows(k).for_each(|window| {
        let mut visited = vec![0; d + 1];
        let mut cnt = 0;

        for i in 0..k {
            if visited[window[i]] == 0 {
                cnt += 1;
            }

            visited[window[i]] += 1;
        }

        if visited[c] == 0 {
            cnt += 1;
        }

        ret = ret.max(cnt);
    });

    writeln!(out, "{ret}").unwrap();
}