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

fn process_tile(
    board: &mut Vec<Vec<i64>>,
    x: usize,
    y: usize,
    size: usize,
    r: usize,
    c: usize,
    id: &mut i64,
) {
    if size == 2 {
        for dy in 0..2 {
            for dx in 0..2 {
                let r_next = y + dy;
                let c_next = x + dx;

                if r_next == r && c_next == c {
                    continue;
                }

                board[r_next][c_next] = *id;
            }
        }

        *id += 1;
        return;
    }

    let half = size / 2;
    let center_r = y + half;
    let center_c = x + half;

    let centers = [
        (center_r - 1, center_c - 1),
        (center_r - 1, center_c),
        (center_r, center_c - 1),
        (center_r, center_c),
    ];
    let quad = match (r >= y + half, c >= x + half) {
        (false, false) => 0,
        (false, true) => 1,
        (true, false) => 2,
        (true, true) => 3,
    };

    for (idx, &(r_next, c_next)) in centers.iter().enumerate() {
        if idx == quad {
            continue;
        }

        board[r_next][c_next] = *id;
    }

    *id += 1;

    for i in 0..4 {
        let y_next = y + (i / 2) * half;
        let x_next = x + (i % 2) * half;
        let (r_next, c_next) = if i == quad { (r, c) } else { centers[i] };

        process_tile(board, x_next, y_next, half, r_next, c_next, id);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<usize>();
    let n = 1usize << k;
    let (x, y) = (scan.token::<usize>() - 1, n - scan.token::<usize>());

    let mut board = vec![vec![0; n]; n];
    board[y][x] = -1;

    let mut id = 1;
    process_tile(&mut board, 0, 0, n, y, x, &mut id);

    for i in 0..n {
        for j in 0..n {
            write!(out, "{} ", board[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
