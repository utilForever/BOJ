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

    let (n, m, t, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut col_sets = vec![Vec::new(); m + 1];
    let mut tile_cols = vec![Vec::new(); t + 1];

    for i in 1..=t {
        let k = scan.token::<usize>();

        for _ in 0..k {
            let x = scan.token::<usize>();
            col_sets[x].push(i);
            tile_cols[i].push(x);
        }
    }

    let stride = m + 1;
    let mut inter = vec![0; stride * stride];

    for i in 1..=t {
        for (j, &c1) in tile_cols[i].iter().enumerate() {
            for &c2 in tile_cols[i][j..].iter() {
                inter[c1 * stride + c2] = i;
                inter[c2 * stride + c1] = i;
            }
        }
    }

    let mut dp = vec![0; stride * stride];

    for i in 1..=m {
        for j in 1..=m {
            let rev = m - j + 1;
            let idx = i * stride + j;
            let diag = (i - 1) * stride + (j - 1);
            let up = (i - 1) * stride + j;
            let left = i * stride + (j - 1);

            dp[idx] = if inter[i * stride + rev] != 0 {
                dp[diag] + 1
            } else {
                dp[up].max(dp[left])
            }
        }
    }

    let lcs = dp[m * stride + m];

    if a + b > n + lcs {
        writeln!(out, "No").unwrap();
        return;
    }

    let mut pairs = Vec::new();
    let (mut i, mut j) = (m, m);

    while i > 0 && j > 0 {
        let rev = m - j + 1;
        let curr = dp[i * stride + j];

        if inter[i * stride + rev] != 0 && curr == dp[(i - 1) * stride + (j - 1)] + 1 {
            pairs.push((i, rev, inter[i * stride + rev]));
            i -= 1;
            j -= 1;
        } else if dp[(i - 1) * stride + j] == curr {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    pairs.reverse();
    pairs.push((m + 1, 0, 0));

    let mut ret = Vec::with_capacity(n);
    let (mut left, mut right) = (0, m + 1);
    let (mut a_have, mut b_have) = (lcs, lcs);

    for &(col_left, col_right, set) in pairs.iter() {
        while left + 1 < col_left && a_have < a {
            left += 1;
            a_have += 1;

            ret.push(col_sets[left][0]);
        }

        while right - 1 > col_right && b_have < b {
            right -= 1;
            b_have += 1;

            ret.push(col_sets[right][0]);
        }

        left = col_left;
        right = col_right;

        if set != 0 {
            ret.push(set);
        }
    }

    while ret.len() < n {
        ret.push(1);
    }

    ret.truncate(n);

    writeln!(out, "Yes").unwrap();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
