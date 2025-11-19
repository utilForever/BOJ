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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut heights = vec![0; n + 1];

    for i in 1..=n {
        heights[i] = scan.token::<i64>();
    }

    let mut edges = vec![(0, 0); m];

    for i in 0..m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());

        if heights[u] < heights[v] {
            edges[i] = (u, v);
        } else {
            edges[i] = (v, u);
        }
    }

    let k = scan.token::<usize>();
    let mut is_opened = vec![false; n + 1];

    for _ in 0..k {
        let a = scan.token::<usize>();
        is_opened[a] = true;
    }

    loop {
        let mut changed = false;

        for &(u, v) in edges.iter() {
            if is_opened[u] && !is_opened[v] {
                is_opened[v] = true;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    writeln!(
        out,
        "{}",
        if is_opened[1..].iter().all(|&x| x) {
            "no flood"
        } else {
            "flood"
        }
    )
    .unwrap();
}
