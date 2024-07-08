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

    let mut chessboard = vec![vec![false; 8]; 8];
    let mut moves = vec![String::new(); 36];

    for i in 0..36 {
        moves[i] = scan.token::<String>();
    }

    let convert_pos = |s: &str| -> (usize, usize) {
        let mut chars = s.chars();
        let x = chars.next().unwrap() as usize - 'A' as usize;
        let y = chars.next().unwrap() as usize - '1' as usize;

        (x, y)
    };
    let can_move = |prev: (usize, usize), curr: (usize, usize)| -> bool {
        let prev = (prev.0 as i32, prev.1 as i32);
        let curr = (curr.0 as i32, curr.1 as i32);

        (prev.0 - curr.0).abs() == 2 && (prev.1 - curr.1).abs() == 1
            || (prev.0 - curr.0).abs() == 1 && (prev.1 - curr.1).abs() == 2
    };

    let mut ret = true;
    let mut prev = convert_pos(&moves[0]);

    chessboard[prev.0][prev.1] = true;

    for i in 1..36 {
        let curr = convert_pos(&moves[i]);

        if chessboard[curr.0][curr.1] {
            ret = false;
            break;
        }

        if can_move(prev, curr) {
            chessboard[curr.0][curr.1] = true;
        } else {
            ret = false;
            break;
        }

        prev = curr;
    }

    // Check can move from the last position to the first position
    if ret {
        ret = can_move(prev, convert_pos(&moves[0]));
    }

    writeln!(out, "{}", if ret { "Valid" } else { "Invalid" }).unwrap();
}
