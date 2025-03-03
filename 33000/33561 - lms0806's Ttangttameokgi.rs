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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

static mut LAND: [[i32; 500]; 500] = [[0; 500]; 500];
static mut BLUEPRINT: [i32; 100_000] = [0; 100_000];
static mut PREFIX_SUM: [[i32; 501]; 501] = [[0; 501]; 501];
static mut PREFIX_ZERO: [[i32; 501]; 501] = [[0; 501]; 501];
static mut PREFIX_BLUEPRINT: [i32; 100_001] = [0; 100_001];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    unsafe {
        let n = scan.token::<usize>();

        for i in 0..n {
            for j in 0..n {
                LAND[i][j] = scan.token::<i32>();
            }
        }

        let k = scan.token::<usize>();

        for i in 0..k {
            BLUEPRINT[i] = scan.token::<i32>();
        }

        BLUEPRINT.sort_unstable_by(|a, b| b.cmp(a));

        for i in 0..n {
            for j in 0..n {
                PREFIX_SUM[i + 1][j + 1] =
                    PREFIX_SUM[i + 1][j] + PREFIX_SUM[i][j + 1] - PREFIX_SUM[i][j] + LAND[i][j];
                PREFIX_ZERO[i + 1][j + 1] = PREFIX_ZERO[i + 1][j] + PREFIX_ZERO[i][j + 1]
                    - PREFIX_ZERO[i][j]
                    + (LAND[i][j] == 0) as i32;
            }
        }

        for i in 0..k {
            PREFIX_BLUEPRINT[i + 1] = PREFIX_BLUEPRINT[i] + BLUEPRINT[i];
        }

        let mut ret = 0;

        for len in 1..=n {
            for i_start in 0..=n - len {
                let i_end = i_start + len;

                for j_start in 0..=n - len {
                    let j_end = j_start + len;

                    let cnt_zero = PREFIX_ZERO[i_end][j_end]
                        - PREFIX_ZERO[i_start][j_end]
                        - PREFIX_ZERO[i_end][j_start]
                        + PREFIX_ZERO[i_start][j_start];

                    if cnt_zero > k as i32 {
                        continue;
                    }

                    let sum = PREFIX_SUM[i_end][j_end]
                        - PREFIX_SUM[i_start][j_end]
                        - PREFIX_SUM[i_end][j_start]
                        + PREFIX_SUM[i_start][j_start];
                    let candidate = sum + PREFIX_BLUEPRINT[cnt_zero as usize];

                    ret = ret.max(candidate);
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
