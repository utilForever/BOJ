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
    let mut board = vec![vec![' '; n]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            board[i][j] = c;
        }
    }

    let mut pos_heart = (0, 0);

    'find_heart: for i in 1..n - 1 {
        for j in 1..n - 1 {
            let left = board[i][j - 1];
            let right = board[i][j + 1];
            let up = board[i - 1][j];
            let down = board[i + 1][j];

            if board[i][j] == '*' && left == '*' && right == '*' && up == '*' && down == '*' {
                pos_heart = (i, j);
                break 'find_heart;
            }
        }
    }

    let mut length_arm_left = 0;
    let mut length_arm_right = 0;
    let mut length_weist = 0;
    let mut length_leg_left = 0;
    let mut length_leg_right = 0;

    let mut idx = pos_heart.1 - 1;

    while board[pos_heart.0][idx] == '*' {
        length_arm_left += 1;

        if idx == 0 {
            break;
        }

        idx -= 1;
    }

    idx = pos_heart.1 + 1;

    while board[pos_heart.0][idx] == '*' {
        length_arm_right += 1;

        if idx == n - 1 {
            break;
        }

        idx += 1;
    }

    idx = pos_heart.0 + 1;

    while board[idx][pos_heart.1] == '*' {
        length_weist += 1;
        idx += 1;
    }

    let pos_weist_end = (idx - 1, pos_heart.1);
    idx = pos_weist_end.0 + 1;

    while board[idx][pos_weist_end.1 - 1] == '*' {
        length_leg_left += 1;

        if idx == n - 1 {
            break;
        }

        idx += 1;
    }

    idx = pos_weist_end.0 + 1;

    while board[idx][pos_weist_end.1 + 1] == '*' {
        length_leg_right += 1;

        if idx == n - 1 {
            break;
        }

        idx += 1;
    }

    writeln!(out, "{} {}", pos_heart.0 + 1, pos_heart.1 + 1).unwrap();
    writeln!(
        out,
        "{length_arm_left} {length_arm_right} {length_weist} {length_leg_left} {length_leg_right}"
    )
    .unwrap();
}
