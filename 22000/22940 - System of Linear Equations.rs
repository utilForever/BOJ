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

const EPS: f64 = 1e-9;

fn gauss_elimination(mut matrix: Vec<Vec<f64>>) -> (i64, Vec<f64>) {
    let n = matrix.len();
    let m = matrix[0].len() - 1;

    let mut pos = vec![-1; m];
    let mut det = 1.0;
    let mut row = 0;

    for col in 0..m {
        if row >= n {
            break;
        }

        let mut row_max = row;

        for i in row..n {
            if matrix[i][col].abs() > matrix[row_max][col].abs() {
                row_max = i;
            }
        }

        if matrix[row_max][col].abs() < EPS {
            det = 0.0;
            continue;
        }

        for i in col..=m {
            let temp = matrix[row][i];
            matrix[row][i] = matrix[row_max][i];
            matrix[row_max][i] = temp;
        }

        if row != row_max {
            det = -det;
        }

        det *= matrix[row][col];
        pos[col] = row as i64;

        for i in 0..n {
            if i != row && matrix[i][col].abs() > EPS {
                let c = matrix[i][col] / matrix[row][col];

                for j in col..=m {
                    matrix[i][j] -= matrix[row][j] * c;
                }
            }
        }

        row += 1;
    }

    let mut ret = vec![0.0; m];

    for i in 0..m {
        if pos[i] != -1 {
            let row = pos[i] as usize;
            ret[i] = matrix[row][m] / matrix[row][i];
        }
    }

    for i in 0..n {
        let mut sum = 0.0;

        for j in 0..m {
            sum += ret[j] * matrix[i][j];
        }

        if (sum - matrix[i][m]).abs() > EPS {
            return (-1, ret);
        }
    }

    for i in 0..m {
        if pos[i] == -1 {
            return (2, ret);
        }
    }

    (1, ret)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut equations = vec![vec![0.0; n + 1]; n];

    for i in 0..n {
        for j in 0..=n {
            equations[i][j] = scan.token::<f64>();
        }
    }

    let (_, ret) = gauss_elimination(equations);

    for i in 0..n {
        write!(out, "{} ", ret[i].round() as i64).unwrap();
    }

    writeln!(out).unwrap();
}
