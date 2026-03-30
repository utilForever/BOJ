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

fn multiply_vector_matrix(vec: &Vec<i64>, mat: &Vec<Vec<i64>>) -> Vec<i64> {
    let mut ret = vec![0; SIZE];

    for i in 0..SIZE {
        for j in 0..SIZE {
            ret[i] = (ret[i] + vec[j] * mat[j][i]) % MOD;
        }
    }

    ret
}

fn multiply_matrix_matrix(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![0; SIZE]; SIZE];

    for i in 0..SIZE {
        for j in 0..SIZE {
            for k in 0..SIZE {
                ret[i][j] = (ret[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }

    ret
}

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 128;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut n, w) = (scan.token::<i64>(), scan.token::<i64>());

    if (w == 1 && n != 0) || (w != 1 && w > n + 1) {
        writeln!(out, "The cake is a hing").unwrap();
        return;
    }

    let mut matrix = vec![vec![0; SIZE]; SIZE];
    let mut vec = vec![0; SIZE];

    for mask in 0..SIZE {
        for x in 1..=7 {
            let mut mask_next = mask | (1usize << (x - 1));

            for y in (1..=x).rev() {
                if (mask & (1usize << (y - 1))) != 0 {
                    mask_next = (mask ^ (1usize << (y - 1))) | (1usize << (x - 1));
                    break;
                }
            }

            matrix[mask][mask_next] += 1;
        }
    }

    vec[0] = 1;

    while n > 0 {
        if n % 2 == 1 {
            vec = multiply_vector_matrix(&vec, &matrix);
        }

        matrix = multiply_matrix_matrix(&matrix, &matrix);
        n /= 2;
    }

    let mut ret = 0;

    for mask in 0..SIZE {
        if mask.count_ones() as i64 == w - 1 {
            ret = (ret + vec[mask]) % MOD;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
