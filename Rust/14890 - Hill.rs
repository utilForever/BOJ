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

fn can_move(n: usize, l: usize, get_height: impl Fn(usize, usize) -> i64) -> bool {
    let mut idx = 0;
    let mut used_hill = vec![false; n];

    while idx < n - 1 {
        let height_curr = get_height(idx, 0);
        let height_next = get_height(idx + 1, 0);

        if height_curr == height_next {
            idx += 1;
        } else if height_curr == height_next + 1 {
            if idx + l >= n {
                return false;
            }

            if (idx + 1..=idx + l).any(|j| get_height(j, 0) != height_next || used_hill[j]) {
                return false;
            }

            (idx + 1..=idx + l).for_each(|j| used_hill[j] = true);
            idx += l;
        } else if height_curr == height_next - 1 {
            if idx < l - 1 {
                return false;
            }

            if (idx - (l - 1)..=idx).any(|j| get_height(j, 0) != height_curr || used_hill[j]) {
                return false;
            }

            (idx - (l - 1)..=idx).for_each(|j| used_hill[j] = true);
            idx += 1;
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, l) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            map[i][j] = scan.token::<i64>();
        }
    }

    let mut ret = 0;

    // Horizontal
    for i in 0..n {
        if can_move(n, l, |idx, _| map[i][idx]) {
            ret += 1;
        }
    }

    // Vertical
    for i in 0..n {
        if can_move(n, l, |idx, _| map[idx][i]) {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
