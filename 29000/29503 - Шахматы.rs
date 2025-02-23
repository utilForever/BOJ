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

    let mut chessboard = [[' '; 8]; 8];

    for i in 0..8 {
        let line = scan.line().trim().to_string();

        for (j, c) in line.trim().chars().enumerate() {
            chessboard[i][j] = c;
        }
    }

    let n = scan.token::<i64>();

    for _ in 0..n {
        let command = scan.line().trim().to_string();
        let command = command.chars().collect::<Vec<_>>();

        let from = (
            command[0] as usize - 'a' as usize,
            8 - (command[1] as usize - '1' as usize + 1),
        );
        let to = (
            command[2] as usize - 'a' as usize,
            8 - (command[3] as usize - '1' as usize + 1),
        );

        write!(out, "{}", chessboard[from.1][from.0]).unwrap();

        chessboard[to.1][to.0] = chessboard[from.1][from.0];
        chessboard[from.1][from.0] = '.';
    }
}
