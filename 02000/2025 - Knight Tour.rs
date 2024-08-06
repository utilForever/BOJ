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
    let mut board = vec![vec![0; n]; n];

    let (mut x, mut y) = (scan.token::<i64>(), scan.token::<i64>());
    let moves = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];
    let mut step = 1;

    board[x as usize - 1][y as usize - 1] = step;
    step += 1;

    while step <= n * n {
        let mut candidates = Vec::new();

        for direction in moves.iter() {
            let (x_next, y_next) = (x + direction.0, y + direction.1);

            if x_next < 0
                || x_next >= n as i64
                || y_next < 0
                || y_next >= n as i64
                || board[x_next as usize][y_next as usize] != 0
            {
                continue;
            }

            let mut cnt = 0;

            for direction in moves.iter() {
                let (x_next_next, y_next_next) = (x_next + direction.0, y_next + direction.1);

                if x_next_next < 0
                    || x_next_next >= n as i64
                    || y_next_next < 0
                    || y_next_next >= n as i64
                    || board[x_next_next as usize][y_next_next as usize] != 0
                {
                    continue;
                }

                cnt += 1;
            }

            candidates.push((cnt, (x_next, y_next)));
        }

        match candidates.iter().min() {
            Some((_, (x_next, y_next))) => {
                x = *x_next;
                y = *y_next;
            }
            None => {
                writeln!(out, "-1 -1").unwrap();
                return;
            }
        }

        board[x as usize][y as usize] = step;
        step += 1;
    }

    let mut board_new = Vec::new();

    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            board_new.push((i + 1, j + 1, cell));
        }
    }

    board_new.sort_by(|a, b| a.2.cmp(&b.2));

    for (i, j, _) in board_new {
        writeln!(out, "{i} {j}").unwrap();
    }
}
