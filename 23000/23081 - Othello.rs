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

    let mut ret: Option<((usize, usize), i32)> = None;

    for y in 0..n {
        for x in 0..n {
            if board[y][x] == 'B' || board[y][x] == 'W' {
                continue;
            }

            let mut cnt = 0;

            // Check 8 directions
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    let mut y_next = y as i32 + dy;
                    let mut x_next = x as i32 + dx;
                    let mut cnt_fliped = 0;

                    loop {
                        if y_next < 0 || y_next >= n as i32 || x_next < 0 || x_next >= n as i32 {
                            cnt_fliped = 0;
                            break;
                        }
                        
                        match board[y_next as usize][x_next as usize] {
                            'W' => cnt_fliped += 1,
                            'B' => break,
                            _ => {
                                cnt_fliped = 0;
                                break;
                            }
                        }

                        y_next += dy;
                        x_next += dx;
                    }

                    cnt += cnt_fliped;
                }
            }

            if cnt > 0 && (ret.is_none() || cnt > ret.unwrap().1) {
                ret = Some(((y, x), cnt));
            }
        }
    }

    match ret {
        Some(val) => {
            let ((y, x), cnt) = val;

            writeln!(out, "{x} {y}").unwrap();
            writeln!(out, "{cnt}").unwrap();
        }
        None => {
            writeln!(out, "PASS").unwrap();
        }
    }
}
