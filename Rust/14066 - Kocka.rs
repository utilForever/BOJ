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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut heights = vec![vec![0; m]; n];
    let mut len_y = 0;
    let len_x = 4 * m + 2 * n + 1;

    for i in 0..n {
        let mut height_max = 0;

        for j in 0..m {
            heights[i][j] = scan.token::<usize>();
            height_max = height_max.max(heights[i][j]);
        }

        len_y = len_y.max(3 * height_max + 3 + 2 * (n - i - 1));
    }

    let mut ret = vec![vec!['.'; len_x]; len_y];

    for i in 0..n {
        for j in 0..m {
            for k in 1..=heights[i][j] {
                let y_start = len_y - 2 * (n - i - 1) - 3 * k + 2;
                let x_start = 2 * (n - i - 1) + 4 * j;
    
                ret[y_start][x_start] = '+';
                ret[y_start][x_start + 1] = '-';
                ret[y_start][x_start + 2] = '-';
                ret[y_start][x_start + 3] = '-';
                ret[y_start][x_start + 4] = '+';
    
                ret[y_start - 1][x_start] = '|';
                ret[y_start - 1][x_start + 1] = ' ';
                ret[y_start - 1][x_start + 2] = ' ';
                ret[y_start - 1][x_start + 3] = ' ';
                ret[y_start - 1][x_start + 4] = '|';
                ret[y_start - 1][x_start + 5] = '/';
    
                ret[y_start - 2][x_start] = '|';
                ret[y_start - 2][x_start + 1] = ' ';
                ret[y_start - 2][x_start + 2] = ' ';
                ret[y_start - 2][x_start + 3] = ' ';
                ret[y_start - 2][x_start + 4] = '|';
                ret[y_start - 2][x_start + 5] = ' ';
                ret[y_start - 2][x_start + 6] = '+';
    
                ret[y_start - 3][x_start] = '+';
                ret[y_start - 3][x_start + 1] = '-';
                ret[y_start - 3][x_start + 2] = '-';
                ret[y_start - 3][x_start + 3] = '-';
                ret[y_start - 3][x_start + 4] = '+';
                ret[y_start - 3][x_start + 5] = ' ';
                ret[y_start - 3][x_start + 6] = '|';
    
                ret[y_start - 4][x_start + 1] = '/';
                ret[y_start - 4][x_start + 2] = ' ';
                ret[y_start - 4][x_start + 3] = ' ';
                ret[y_start - 4][x_start + 4] = ' ';
                ret[y_start - 4][x_start + 5] = '/';
                ret[y_start - 4][x_start + 6] = '|';
    
                ret[y_start - 5][x_start + 2] = '+';
                ret[y_start - 5][x_start + 3] = '-';
                ret[y_start - 5][x_start + 4] = '-';
                ret[y_start - 5][x_start + 5] = '-';
                ret[y_start - 5][x_start + 6] = '+';
            }
        }
    }
    
    for i in 0..len_y {
        for j in 0..len_x {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
