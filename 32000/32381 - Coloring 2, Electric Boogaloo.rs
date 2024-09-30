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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<i64>(), scan.token::<usize>());
    let mut cnt_black = 0;
    let mut row = 0;
    let mut col = 0;
    let mut ret = vec![(' ', 0); q];

    for i in 0..q {
        let cnt_black_curr = scan.token::<i64>();
        let cnt_black_row_add = cnt_black + n - 2 * col;
        let cnt_black_row_sub = cnt_black - n + 2 * col;
        let cnt_black_col_add = cnt_black + n - 2 * row;
        let cnt_black_col_sub = cnt_black - n + 2 * row;

        if cnt_black_curr == cnt_black_row_add {
            ret[i] = ('R', row + 1);
            row += 1;
        } else if cnt_black_curr == cnt_black_row_sub {
            row -= 1;
            ret[i] = ('R', row + 1);
        } else if cnt_black_curr == cnt_black_col_add {
            ret[i] = ('C', col + 1);
            col += 1;
        } else if cnt_black_curr == cnt_black_col_sub {
            col -= 1;
            ret[i] = ('C', col + 1);
        } else {
            writeln!(out, "-1").unwrap();
            return;
        }

        if row < 0 || row > n || col < 0 || col > n {
            writeln!(out, "-1").unwrap();
            return;
        }

        cnt_black = cnt_black_curr;
    }

    for (c, idx) in ret {
        writeln!(out, "{c} {idx}").unwrap();
    }
}
