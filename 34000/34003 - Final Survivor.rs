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

    let mut board = [[' '; 8]; 8];

    for i in 0..8 {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            board[i][j] = c;
        }
    }

    let mut comb = [[0; 5]; 65];

    for i in 1..=64 {
        comb[i][0] = 1;

        for j in 1..=4 {
            if j > i {
                continue;
            }

            comb[i][j] = if i == j {
                1
            } else {
                comb[i - 1][j - 1] + comb[i - 1][j]
            };
        }
    }

    let cnt_block_alive_total = board
        .iter()
        .map(|row| row.iter().filter(|&&c| c == 'O').count())
        .sum::<usize>();
    let denominator = comb[cnt_block_alive_total][4];
    let mut ret = (0, 0, 0.0);

    for i in 0..7 {
        for j in 0..7 {
            let mut cnt_block_alive = 0;

            for dy in 0..=1 {
                for dx in 0..=1 {
                    if board[i + dy][j + dx] == 'O' {
                        cnt_block_alive += 1;
                    }
                }
            }

            if cnt_block_alive == 0 {
                continue;
            }

            let numerator =
                comb[cnt_block_alive_total][4] - comb[cnt_block_alive_total - cnt_block_alive][4];
            let probability = numerator as f64 / denominator as f64;

            if probability > ret.2 {
                ret = (i, j, probability);
            }
        }
    }

    writeln!(out, "{} {}", ret.0 + 1, ret.1 + 1).unwrap();
    writeln!(out, "{:.12}", ret.2).unwrap();
}
