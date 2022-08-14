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

fn calculate(dp: &mut [[[i32; 8]; 512]; 10], board: &[i32; 10], x: usize, y: usize, z: usize) -> i32 {
    if x == 0 {
        if y == 0 {
            return 0;
        } else {
            return 1_000_000_007;
        }
    }

    if x % 3 == 0 && z != 0 {
        return 1_000_000_007;
    }

    if (dp[x][y][z] ^ -1) != 0 {
        return dp[x][y][z];
    }

    let mut ret = 1_000_000_007;

    for i in (0..=512).rev() {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;

        for j in 0..9 {
            if (1 << j & i) != 0 {
                a ^= 1 << j / 3;
                b += 1;
            }
        }

        if b & 1 != 0 {
           continue;
        }

        for j in 0..9 {
            if (1 << j & (i ^ board[x])) != 0 {
                c += 1;
            }
        }

        let x_new = x - 1;
        let y_new = y ^ (i as usize);
        let z_new = z ^ (a as usize);

        if x_new < 10 && y_new < 512 && z_new < 8 {
            ret = std::cmp::min(ret, calculate(dp, board, x_new, y_new, z_new) + c);
        }
    }

    dp[x][y][z] = ret;

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut board = [0; 10];

    for i in 1..=9 {
        let s = scan.token::<String>();

        for c in s.chars() {
            let val = (c as u8 - '0' as u8) as i32;
            board[i] = board[i] * 2 + val;
        }
    }

    let mut dp = [[[-1; 8]; 512]; 10];

    writeln!(out, "{}", calculate(&mut dp, &board, 9, 0, 0)).unwrap();
}
