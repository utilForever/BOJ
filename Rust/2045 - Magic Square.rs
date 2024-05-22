use io::Write;
use std::{io, str, vec};

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

    let mut magic_square = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            magic_square[i][j] = scan.token::<i64>();
        }
    }

    // Case that all elements of diagonal are 0
    if magic_square[0][0] == 0 && magic_square[1][1] == 0 && magic_square[2][2] == 0 {
        let first = magic_square[0][1] + magic_square[0][2];
        let second = magic_square[1][0] + magic_square[1][2];
        let third = magic_square[2][0] + magic_square[2][1];

        magic_square[0][0] = ((second + third) - first) / 2;
        magic_square[1][1] = ((first + third) - second) / 2;
        magic_square[2][2] = ((first + second) - third) / 2;
    } else if magic_square[0][2] == 0 && magic_square[1][1] == 0 && magic_square[2][0] == 0 {
        let first = magic_square[0][0] + magic_square[0][1];
        let second = magic_square[1][0] + magic_square[1][2];
        let third = magic_square[2][1] + magic_square[2][2];

        magic_square[0][2] = ((second + third) - first) / 2;
        magic_square[1][1] = ((first + third) - second) / 2;
        magic_square[2][0] = ((first + second) - third) / 2;
    }

    let mut sum = 0;

    // Horizontal
    for i in 0..3 {
        if magic_square[i][0] != 0 && magic_square[i][1] != 0 && magic_square[i][2] != 0 {
            sum = magic_square[i][0] + magic_square[i][1] + magic_square[i][2];
            break;
        }
    }

    // Vertical
    for i in 0..3 {
        if magic_square[0][i] != 0 && magic_square[1][i] != 0 && magic_square[2][i] != 0 {
            sum = magic_square[0][i] + magic_square[1][i] + magic_square[2][i];
            break;
        }
    }

    // Diagonal
    if magic_square[0][0] != 0 && magic_square[1][1] != 0 && magic_square[2][2] != 0 {
        sum = magic_square[0][0] + magic_square[1][1] + magic_square[2][2];
    }

    if magic_square[0][2] != 0 && magic_square[1][1] != 0 && magic_square[2][0] != 0 {
        sum = magic_square[0][2] + magic_square[1][1] + magic_square[2][0];
    }

    for i in 0..3 {
        for j in 0..3 {
            if magic_square[i][j] != 0 {
                continue;
            }

            magic_square[i][j] =
                if magic_square[i][(j + 1) % 3] != 0 && magic_square[i][(j + 2) % 3] != 0 {
                    sum - magic_square[i][(j + 1) % 3] - magic_square[i][(j + 2) % 3]
                } else if magic_square[(i + 1) % 3][j] != 0 && magic_square[(i + 2) % 3][j] != 0 {
                    sum - magic_square[(i + 1) % 3][j] - magic_square[(i + 2) % 3][j]
                } else {
                    sum - magic_square[(i + 1) % 3][(j + 1) % 3]
                        - magic_square[(i + 2) % 3][(j + 2) % 3]
                };
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            write!(out, "{} ", magic_square[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
