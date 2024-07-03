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

    let mut n = scan.token::<usize>();
    let mut nums = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            nums[i][j] = scan.token::<i64>();
        }
    }

    while n > 1 {
        let mut nums_new = vec![vec![0; n / 2]; n / 2];

        for i in 0..n / 2 {
            for j in 0..n / 2 {
                let mut nums_cell = vec![0; 4];
                nums_cell[0] = nums[i * 2][j * 2];
                nums_cell[1] = nums[i * 2][j * 2 + 1];
                nums_cell[2] = nums[i * 2 + 1][j * 2];
                nums_cell[3] = nums[i * 2 + 1][j * 2 + 1];

                nums_cell.sort();

                nums_new[i][j] = nums_cell[2];
            }
        }

        n /= 2;
        nums = nums_new;
    }

    writeln!(out, "{}", nums[0][0]).unwrap();
}
