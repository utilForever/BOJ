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

const MOD: i32 = 9901;

// Define a recursive/closure function for backtracking
// row: current row index in the column (0 ~ h)
// mask: bitmask of the current column's occupied cells
// mask_next: bitmask of the next column's occupied cells
// ways: the number of ways accumulated from dp[c][mask]
fn process(row: usize, h: usize, mask: usize, mask_next: usize, ways: i32, dp_next: &mut Vec<i32>) {
    // If we've checked all rows in the current column, update dp_next for mask_next
    if row == h {
        dp_next[mask_next] = (dp_next[mask_next] + ways) % MOD;
        return;
    }

    // If the current row is already occupied in 'mask', skip to the next row
    if (mask & (1 << row)) != 0 {
        process(row + 1, h, mask, mask_next, ways, dp_next);
    } else {
        // (1) Place a vertical domino: Ensure row+1 < h and that row+1 is not occupied
        if row + 1 < h && (mask & (1 << (row + 1))) == 0 {
            let mask_new = mask | (1 << row) | (1 << (row + 1));
            process(row + 2, h, mask_new, mask_next, ways, dp_next);
        }

        // (2) Place a horizontal domino: Ensure the corresponding cell in the next column is free
        if (mask_next & (1 << row)) == 0 {
            let mask_new = mask | (1 << row);
            let mask_next_new = mask_next | (1 << row);
            process(row + 1, h, mask_new, mask_next_new, ways, dp_next);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    // If the total number of cells is odd, it can't be fully tiled with 2x1 dominos
    if (n * m) % 2 != 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    let (h, w) = if n < m { (n, m) } else { (m, n) };
    let mut dp = vec![0; 1 << h];
    let mut dp_next = vec![0; 1 << h];

    dp[0] = 1;

    for _ in 0..w {
        dp_next.fill(0);

        for mask in 0..(1 << h) {
            if dp[mask] == 0 {
                continue;
            }

            // For each possible mask in the current dp, explore placements
            process(0, h, mask, 0, dp[mask], &mut dp_next);
        }

        dp.copy_from_slice(&dp_next);
    }

    writeln!(out, "{}", dp[0] % MOD).unwrap();
}
