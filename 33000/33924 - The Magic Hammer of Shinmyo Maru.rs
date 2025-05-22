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

    let mut bowls = [[1, 2], [3, 4], [5, 6], [7, 8]];
    let (n, m) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    let num_bowl = bowls[n][m];

    let _ = scan.token::<i64>();
    let s = scan.token::<String>();

    for c in s.chars() {
        let mut bowls_new = [[0; 2]; 4];

        if c == 'A' {
            bowls_new[0][0] = bowls[2][0];
            bowls_new[0][1] = bowls[2][1];
            bowls_new[1][0] = bowls[3][0];
            bowls_new[1][1] = bowls[3][1];
            bowls_new[2][0] = bowls[0][0];
            bowls_new[2][1] = bowls[0][1];
            bowls_new[3][0] = bowls[1][0];
            bowls_new[3][1] = bowls[1][1];
        } else if c == 'B' {
            bowls_new[0][0] = bowls[1][1];
            bowls_new[0][1] = bowls[1][0];
            bowls_new[1][0] = bowls[0][1];
            bowls_new[1][1] = bowls[0][0];
            bowls_new[2][0] = bowls[3][1];
            bowls_new[2][1] = bowls[3][0];
            bowls_new[3][0] = bowls[2][1];
            bowls_new[3][1] = bowls[2][0];
        } else if c == 'C' {
            bowls_new[0][0] = bowls[3][1];
            bowls_new[0][1] = bowls[3][0];
            bowls_new[1][0] = bowls[2][1];
            bowls_new[1][1] = bowls[2][0];
            bowls_new[2][0] = bowls[1][1];
            bowls_new[2][1] = bowls[1][0];
            bowls_new[3][0] = bowls[0][1];
            bowls_new[3][1] = bowls[0][0];
        } else {
            bowls_new[0][0] = bowls[1][0];
            bowls_new[0][1] = bowls[0][0];
            bowls_new[1][0] = bowls[2][0];
            bowls_new[1][1] = bowls[0][1];
            bowls_new[2][0] = bowls[3][0];
            bowls_new[2][1] = bowls[1][1];
            bowls_new[3][0] = bowls[3][1];
            bowls_new[3][1] = bowls[2][1];
        }

        std::mem::swap(&mut bowls, &mut bowls_new);
    }

    let mut ret = (0, 0);

    'outer: for i in 0..4 {
        for j in 0..2 {
            if bowls[i][j] == num_bowl {
                ret = (i, j);
                break 'outer;
            }
        }
    }

    writeln!(out, "{} {}", ret.0 + 1, ret.1 + 1).unwrap();
}
