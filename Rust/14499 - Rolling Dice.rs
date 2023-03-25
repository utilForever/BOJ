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

    let dx = [0, 0, -1, 1];
    let dy = [1, -1, 0, 0];
    let mut dice = [0; 7];

    let (n, m, mut x, mut y, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut map = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            map[i][j] = scan.token::<i64>();
        }
    }

    for _ in 0..k {
        let command = scan.token::<usize>();
        let x_next = x as i64 + dx[command - 1];
        let y_next = y as i64 + dy[command - 1];

        if x_next < 0 || x_next >= n as i64 || y_next < 0 || y_next >= m as i64 {
            continue;
        }

        x = x_next as usize;
        y = y_next as usize;

        match command {
            // East
            1 => {
                dice = [0, dice[4], dice[2], dice[1], dice[6], dice[5], dice[3]];
            }
            // West
            2 => {
                dice = [0, dice[3], dice[2], dice[6], dice[1], dice[5], dice[4]];
            }
            // North
            3 => {
                dice = [0, dice[5], dice[1], dice[3], dice[4], dice[6], dice[2]];
            }
            // South
            4 => {
                dice = [0, dice[2], dice[6], dice[3], dice[4], dice[1], dice[5]];
            }
            _ => unreachable!(),
        }

        writeln!(out, "{}", dice[1]).unwrap();

        if map[x][y] == 0 {
            map[x][y] = dice[6];
        } else {
            dice[6] = map[x][y];
            map[x][y] = 0;
        }
    }
}
