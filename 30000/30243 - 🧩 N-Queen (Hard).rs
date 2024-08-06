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

fn backtrack(
    next: &Vec<usize>,
    n: usize,
    m: usize,
    alive: &mut u32,
    diag_left: &mut u64,
    diag_right: &mut u64,
    ret: &mut Vec<u32>,
) -> bool {
    if m >= n {
        return true;
    }

    let mut temp = *alive & !(((*diag_left >> m) | (*diag_right >> (n - m - 1))) as u32);
    temp = (temp << 16) | (temp >> 16);

    while temp > 0 {
        let a = ((temp as i32) & (-(temp as i32))) as u32;
        let b = if a < (1 << 16) { a << 16 } else { a >> 16 };

        *alive ^= b;
        *diag_left ^= (b as u64) << m;
        *diag_right ^= (b as u64) << (n - m - 1);

        if backtrack(next, n, next[m], alive, diag_left, diag_right, ret) {
            ret[m] = b;
            return true;
        }

        *alive ^= b;
        *diag_left ^= (b as u64) << m;
        *diag_right ^= (b as u64) << (n - m - 1);

        temp ^= a;
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut queen = vec![0; n];
    let mut alive = (1 << n) - 1;

    for i in 0..n {
        queen[i] = scan.token::<i32>();
    }

    let mut diag_left = 0_u64;
    let mut diag_right = 0_u64;
    let mut next = vec![0; n];
    let mut ret = vec![0; n];

    for (idx, pos) in queen.iter().enumerate() {
        if *pos > 0 {
            ret[idx] = 1 << (pos - 1);
            alive ^= 1 << (pos - 1);
            diag_left |= 1 << (pos - 1 + idx as i32) as u64;
            diag_right |= 1 << (pos - 1 + n as i32 - 1 - idx as i32) as u64;
        }
    }

    for i in 0..n {
        if queen[i] > 0 {
            continue;
        }

        let mut offset = 1;

        while i + offset < n && queen[i + offset] > 0 {
            offset += 1;
        }

        next[i] = i + offset;
    }

    let mut start = 0;

    while start < n && queen[start] > 0 {
        start += 1;
    }

    if !backtrack(
        &next,
        n,
        start,
        &mut alive,
        &mut diag_left,
        &mut diag_right,
        &mut ret,
    ) {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 0..n {
        let mut idx = 0;

        while (1 << idx) < ret[i] {
            idx += 1;
        }

        write!(out, "{} ", idx + 1).unwrap();
    }

    writeln!(out).unwrap();
}
