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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut board = [[' '; 2]; 2];
    let a = scan.token::<String>();

    for (idx, c) in a.chars().enumerate() {
        board[0][idx] = c;
    }

    let b = scan.token::<String>();

    for (idx, c) in b.chars().enumerate() {
        board[1][idx] = c;
    }

    let mut candidates = Vec::with_capacity(8);

    candidates.push((board[0][0], board[0][1], board[1][1], board[1][0]));
    candidates.push((board[0][0], board[1][0], board[1][1], board[0][1]));
    candidates.push((board[0][1], board[1][1], board[1][0], board[0][0]));
    candidates.push((board[0][1], board[0][0], board[1][0], board[1][1]));
    candidates.push((board[1][1], board[1][0], board[0][0], board[0][1]));
    candidates.push((board[1][1], board[0][1], board[0][0], board[1][0]));
    candidates.push((board[1][0], board[0][0], board[0][1], board[1][1]));
    candidates.push((board[1][0], board[1][1], board[0][1], board[0][0]));

    writeln!(
        out,
        "{}",
        if candidates
            .iter()
            .any(|&(a, b, c, d)| a == 'H' && b == 'E' && c == 'P' && d == 'C')
        {
            "YES"
        } else {
            "NO"
        }
    )
    .unwrap();
}
