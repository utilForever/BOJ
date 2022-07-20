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

fn sum(tree: &Vec<Vec<i64>>, y: usize, x: usize) -> i64 {
    let mut sum = 0;
    let mut x = x as i64;

    if y == 0 {
        return 1;
    }

    while x > 0 {
        sum = (sum + tree[y][x as usize]) % 1_000_000_007;
        x -= x & -x;
    }

    sum
}

fn update(tree: &mut Vec<Vec<i64>>, y: usize, x: usize, diff: i64) {
    let mut x = x as i64;

    while x < tree[0].len() as i64 {
        tree[y][x as usize] = (tree[y][x as usize] + diff) % 1_000_000_007;
        x += x & -x;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), 11);

    let mut arr = vec![0; n + 1];
    let mut tree = vec![vec![0; n + 1]; k + 1];

    for i in 1..=n {
        arr[i] = scan.token();
    }

    let mut cnt = vec![vec![0; n + 1]; k + 1];

    for i in 1..=n {
        for j in 1..=k {
            cnt[j][i] = sum(&tree, j - 1, arr[i] - 1);
            update(&mut tree, j, arr[i], cnt[j][i]);
        }
    }

    let mut ret = 0;

    for i in 1..=n {
        ret = (ret + cnt[k][i]) % 1_000_000_007;
    }

    writeln!(out, "{}", ret).unwrap();
}
