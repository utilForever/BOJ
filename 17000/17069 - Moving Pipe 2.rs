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

    let n = scan.token::<usize>();
    let mut house = vec![vec![0; n + 1]; n + 1];

    for i in 0..n {
        for j in 0..n {
            house[i][j] = scan.token::<usize>();
        }
    }

    let mut num_cases = vec![vec![vec![0_usize; 3]; n + 1]; n + 1];
    num_cases[0][1][0] = 1;

    for i in 0..n {
        for j in 0..n {
            if house[i][j + 1] == 0 {
                num_cases[i][j + 1][0] += num_cases[i][j][0] + num_cases[i][j][2];
            }

            if house[i + 1][j] == 0 {
                num_cases[i + 1][j][1] += num_cases[i][j][1] + num_cases[i][j][2];
            }

            if house[i + 1][j + 1] == 0 && house[i + 1][j] == 0 && house[i][j + 1] == 0 {
                num_cases[i + 1][j + 1][2] +=
                    num_cases[i][j][0] + num_cases[i][j][1] + num_cases[i][j][2];
            }
        }
    }

    writeln!(
        out,
        "{}",
        num_cases[n - 1][n - 1][0] + num_cases[n - 1][n - 1][1] + num_cases[n - 1][n - 1][2]
    )
    .unwrap();
}
