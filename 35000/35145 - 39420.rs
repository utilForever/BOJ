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
    let mut grid = vec![vec!['0'; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let mut ret = 0;

    for i in 0..n {
        let mut masks = vec![0; m];
        let mut is_valid = vec![true; m];

        for j in i..(i + 10).min(n) {
            for k in 0..m {
                if is_valid[k] {
                    let bit = 1u16 << (grid[j][k] as u32);

                    if (masks[k] & bit) != 0 {
                        is_valid[k] = false;
                    } else {
                        masks[k] |= bit;
                    }
                }
            }

            let mut left = 0;
            let mut val = 0;

            for right in 0..m {
                if !is_valid[right] {
                    left = right + 1;
                    val = 0;
                    continue;
                }

                while (val & masks[right]) != 0 {
                    val ^= masks[left];
                    left += 1;
                }

                val |= masks[right];
                ret += right - left + 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
