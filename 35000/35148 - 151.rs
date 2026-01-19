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

fn match_idx(
    i: usize,
    j: usize,
    col_hash1: &Vec<u64>,
    col_hash2: &Vec<u64>,
    col_rotate_hash1: &Vec<u64>,
    col_rotate_hash2: &Vec<u64>,
) -> bool {
    col_hash1[i] == col_rotate_hash1[j] && col_hash2[i] == col_rotate_hash2[j]
}

fn count_palindromes(
    col_hash1: &Vec<u64>,
    col_hash2: &Vec<u64>,
    col_rotate_hash1: &Vec<u64>,
    col_rotate_hash2: &Vec<u64>,
    radius1: &mut Vec<usize>,
    radius2: &mut Vec<usize>,
    self_ok: &mut Vec<bool>,
) -> i64 {
    let n = col_hash1.len() as isize;

    for i in 0..n {
        let idx = i as usize;
        self_ok[idx] =
            col_hash1[idx] == col_rotate_hash1[idx] && col_hash2[idx] == col_rotate_hash2[idx];
    }

    let mut l = 0;
    let mut r = -1;

    for i in 0..n {
        if !self_ok[i as usize] {
            radius1[i as usize] = 0;
            continue;
        }

        let mut k = if i > r {
            1
        } else {
            radius1[(l + r - i) as usize].min((r - i + 1) as usize) as isize
        };

        while i - k >= 0
            && i + k < n
            && match_idx(
                (i - k) as usize,
                (i + k) as usize,
                col_hash1,
                col_hash2,
                col_rotate_hash1,
                col_rotate_hash2,
            )
        {
            k += 1;
        }

        radius1[i as usize] = k as usize;

        if i + k - 1 > r {
            l = i - k + 1;
            r = i + k - 1;
        }
    }

    let mut odd = 0;

    for i in 0..n {
        odd += radius1[i as usize] as i64;
    }

    l = 0;
    r = -1;

    for i in 0..n {
        let mut k = if i > r {
            0
        } else {
            radius2[(l + r - i + 1) as usize].min((r - i + 1) as usize) as isize
        };

        while i - k - 1 >= 0
            && i + k < n
            && match_idx(
                (i - k - 1) as usize,
                (i + k) as usize,
                col_hash1,
                col_hash2,
                col_rotate_hash1,
                col_rotate_hash2,
            )
        {
            k += 1;
        }

        radius2[i as usize] = k as usize;

        if i + k - 1 > r {
            l = i - k;
            r = i + k - 1;
        }
    }

    let mut even = 0;

    for i in 0..n {
        even += radius2[i as usize] as i64;
    }

    odd + even
}

const BASE1: u64 = 0x9E3779B97F4A7C15;
const BASE2: u64 = 0xBF58476D1CE4E5B9;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![0; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c as u8 - b'0';
        }
    }

    let mut rotation = [10u64; 10];
    rotation[0] = 0;
    rotation[1] = 1;
    rotation[2] = 2;
    rotation[5] = 5;
    rotation[6] = 9;
    rotation[8] = 8;
    rotation[9] = 6;

    let mut pow1 = vec![0u64; n + 1];
    let mut pow2 = vec![0u64; n + 1];

    pow1[0] = 1;
    pow2[0] = 1;

    for i in 1..=n {
        pow1[i] = pow1[i - 1].wrapping_mul(BASE1);
        pow2[i] = pow2[i - 1].wrapping_mul(BASE2);
    }

    let mut radius1 = vec![0; m];
    let mut radius2 = vec![0; m];
    let mut self_ok = vec![false; m];
    let mut ret = 0;

    for top in 0..n {
        let mut col_hash1 = vec![0u64; m];
        let mut col_hash2 = vec![0u64; m];
        let mut col_rotate_hash1 = vec![0u64; m];
        let mut col_rotate_hash2 = vec![0u64; m];
        let mut height = 0;

        for bottom in top..n {
            let row = &grid[bottom];

            for i in 0..m {
                col_hash1[i] = col_hash1[i].wrapping_mul(BASE1).wrapping_add(row[i] as u64);
                col_hash2[i] = col_hash2[i].wrapping_mul(BASE2).wrapping_add(row[i] as u64);
                col_rotate_hash1[i] = col_rotate_hash1[i]
                    .wrapping_add(rotation[row[i] as usize].wrapping_mul(pow1[height]));
                col_rotate_hash2[i] = col_rotate_hash2[i]
                    .wrapping_add(rotation[row[i] as usize].wrapping_mul(pow2[height]));
            }

            height += 1;
            ret += count_palindromes(
                &col_hash1,
                &col_hash2,
                &col_rotate_hash1,
                &col_rotate_hash2,
                &mut radius1,
                &mut radius2,
                &mut self_ok,
            );
        }
    }

    writeln!(out, "{ret}").unwrap();
}
