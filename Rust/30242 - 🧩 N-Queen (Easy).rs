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

fn process_dfs(
    queen: &mut Vec<usize>,
    check_column: &mut Vec<bool>,
    check_diagonal1: &mut Vec<bool>,
    check_diagonal2: &mut Vec<bool>,
    idx_row: usize,
    n: usize,
) -> bool {
    if idx_row == n + 1 {
        return true;
    }

    if queen[idx_row] != 0 {
        return process_dfs(
            queen,
            check_column,
            check_diagonal1,
            check_diagonal2,
            idx_row + 1,
            n,
        );
    }

    for idx_column in 1..=n {
        let diagonal1 = idx_row + n - idx_column - 1;
        let diagonal2 = idx_row + idx_column;

        if check_column[idx_column] || check_diagonal1[diagonal1] || check_diagonal2[diagonal2] {
            continue;
        }

        queen[idx_row] = idx_column;
        check_column[idx_column] = true;
        check_diagonal1[diagonal1] = true;
        check_diagonal2[diagonal2] = true;

        if process_dfs(
            queen,
            check_column,
            check_diagonal1,
            check_diagonal2,
            idx_row + 1,
            n,
        ) {
            return true;
        }

        queen[idx_row] = 0;
        check_column[idx_column] = false;
        check_diagonal1[diagonal1] = false;
        check_diagonal2[diagonal2] = false;
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut queen = vec![0; n + 1];
    let mut check_column = vec![false; n + 1];
    let mut check_diagonal1 = vec![false; 2 * n + 1];
    let mut check_diagonal2 = vec![false; 2 * n + 1];

    for idx_row in 1..=n {
        let idx_column = scan.token::<usize>();

        if idx_column == 0 {
            continue;
        }

        queen[idx_row] = idx_column;

        let idx_diagonal1 = idx_row + n - idx_column - 1;
        let idx_diagonal2 = idx_row + idx_column;

        check_column[idx_column] = true;
        check_diagonal1[idx_diagonal1] = true;
        check_diagonal2[idx_diagonal2] = true;
    }

    let ret = process_dfs(
        &mut queen,
        &mut check_column,
        &mut check_diagonal1,
        &mut check_diagonal2,
        1,
        n,
    );

    if ret {
        for idx_column in queen.iter().skip(1) {
            write!(out, "{idx_column} ").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
