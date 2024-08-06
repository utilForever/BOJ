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

    let (pos_king, pos_stone, n) = (
        scan.token::<String>(),
        scan.token::<String>(),
        scan.token::<usize>(),
    );
    let mut x_pos_king = (pos_king.as_bytes()[0] - b'A') as i64;
    let mut y_pos_king = (b'8' - pos_king.as_bytes()[1]) as i64;
    let mut x_pos_stone = (pos_stone.as_bytes()[0] - b'A') as i64;
    let mut y_pos_stone = (b'8' - pos_stone.as_bytes()[1]) as i64;

    let dx = [1, -1, 0, 0, 1, -1, 1, -1];
    let dy = [0, 0, 1, -1, -1, -1, 1, 1];

    for _ in 0..n {
        let direction = scan.token::<String>();
        let idx = if direction == "R" {
            0
        } else if direction == "L" {
            1
        } else if direction == "B" {
            2
        } else if direction == "T" {
            3
        } else if direction == "RT" {
            4
        } else if direction == "LT" {
            5
        } else if direction == "RB" {
            6
        } else {
            7
        };

        let next_x_pos_king = x_pos_king + dx[idx];
        let next_y_pos_king = y_pos_king + dy[idx];

        if next_x_pos_king < 0
            || next_x_pos_king >= 8
            || next_y_pos_king < 0
            || next_y_pos_king >= 8
        {
            continue;
        }

        if next_x_pos_king == x_pos_stone && next_y_pos_king == y_pos_stone {
            let next_x_pos_stone = x_pos_stone + dx[idx];
            let next_y_pos_stone = y_pos_stone + dy[idx];

            if next_x_pos_stone < 0
                || next_x_pos_stone >= 8
                || next_y_pos_stone < 0
                || next_y_pos_stone >= 8
            {
                continue;
            }

            x_pos_stone = next_x_pos_stone;
            y_pos_stone = next_y_pos_stone;
        }

        x_pos_king = next_x_pos_king;
        y_pos_king = next_y_pos_king;
    }

    writeln!(
        out,
        "{}{}",
        (x_pos_king + 'A' as i64) as u8 as char,
        (b'8' - y_pos_king as u8) as char
    )
    .unwrap();
    writeln!(
        out,
        "{}{}",
        (x_pos_stone + 'A' as i64) as u8 as char,
        (b'8' - y_pos_stone as u8) as char
    )
    .unwrap();
}
