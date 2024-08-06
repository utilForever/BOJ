use io::Write;
use std::{collections::VecDeque, io, str};

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

fn process_bfs(board: &Vec<usize>, count: &mut Vec<usize>) {
    let mut queue = VecDeque::new();

    for i in 2..=7 {
        queue.push_back(board[i]);
        count[board[i]] = 1;
    }

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if pos == 100 {
            break;
        }

        for i in 1..=6 {
            if pos + i > 100 {
                continue;
            }

            if count[board[pos + i]] == 0 {
                queue.push_back(board[pos + i]);
                count[board[pos + i]] = count[pos] + 1;
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = vec![0; 101];
    let mut count = vec![0; 101];

    for i in 1..=100 {
        board[i] = i;
    }

    for _ in 0..(n + m) {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        board[x] = y;
    }

    process_bfs(&board, &mut count);

    writeln!(out, "{}", count[100]).unwrap();
}
