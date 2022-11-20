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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut queens = vec![(0, 0); k];

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());

    for i in 0..k {
        queens[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let is_check_cur_turn = queens
        .iter()
        .any(|(x, y)| *x == r || *y == c || (x - y) == (r - c) || (x + y) == (r + c));

    let dx = [-1, 1, 0, 0, -1, -1, 1, 1];
    let dy = [0, 0, -1, 1, -1, 1, -1, 1];
    let mut is_check_next_turn = true;

    for i in 0..8 {
        let (r_next, c_next) = (r as i32 + dx[i], c as i32 + dy[i]);

        if r_next < 1 || r_next > n as i32 || c_next < 1 || c_next > n as i32 {
            continue;
        }

        let ret = queens.iter().any(|(x, y)| {
            *x == r_next as usize
                || *y == c_next as usize
                || (*x - *y) == (r_next - c_next) as usize
                || (*x + *y) == (r_next + c_next) as usize
        });

        if !ret {
            is_check_next_turn = false;
            break;
        }
    }

    writeln!(
        out,
        "{}",
        if is_check_cur_turn && is_check_next_turn {
            "CHECKMATE"
        } else if is_check_cur_turn {
            "CHECK"
        } else if is_check_next_turn {
            "STALEMATE"
        } else {
            "NONE"
        }
    )
    .unwrap();
}
