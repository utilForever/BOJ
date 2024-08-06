use io::Write;
use std::{cmp, io, str};

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

    let n = scan.token::<i64>();
    let mut coins = vec![vec![0; 3]; n as usize + 1];
    let mut ret = 0;

    for _ in 1..=2 * n {
        let (mut x, mut y) = (scan.token::<i64>(), scan.token::<i64>());

        if x < 1 {
            ret += 1 - x;
            x = 1;
        } else if x > n {
            ret += x - n;
            x = n;
        }

        if y < 1 {
            ret += 1 - y;
            y = 1;
        } else if y > 2 {
            ret += y - 2;
            y = 2;
        }

        coins[x as usize][y as usize] += 1;
    }

    let mut operations = vec![0_i64; 3];

    for i in 1..=n as usize {
        operations[1] += coins[i][1] - 1;
        operations[2] += coins[i][2] - 1;

        if operations[1] * operations[2] < 0 {
            let operations_min = cmp::min(operations[1].abs(), operations[2].abs());
            ret += operations_min;

            for j in 1..=2 {
                operations[j] =
                    (operations[j].abs() - operations_min) * (operations[j] / operations[j].abs());
            }
        }

        ret += operations[1].abs() + operations[2].abs();
    }

    writeln!(out, "{}", ret).unwrap();
}
