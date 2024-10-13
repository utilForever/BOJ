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

enum GameResult {
    XWon,
    OWon,
    Draw,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let check_result = |x: i64, o: i64| -> GameResult {
        if x == 4 {
            GameResult::XWon
        } else if o == 4 {
            GameResult::OWon
        } else {
            GameResult::Draw
        }
    };

    for i in 1..=t {
        let mut board = [[' '; 4]; 4];

        for j in 0..4 {
            let s = scan.token::<String>();

            for (k, c) in s.chars().enumerate() {
                board[j][k] = c;
            }
        }

        let mut ret = GameResult::Draw;

        for j in 0..4 {
            let mut x = 0;
            let mut o = 0;

            // Check horizontal
            for k in 0..4 {
                match board[j][k] {
                    'X' => x += 1,
                    'O' => o += 1,
                    'T' => {
                        x += 1;
                        o += 1;
                    }
                    _ => {}
                }
            }

            ret = check_result(x, o);

            if !matches!(ret, GameResult::Draw) {
                break;
            }

            x = 0;
            o = 0;

            // Check vertical
            for k in 0..4 {
                match board[k][j] {
                    'X' => x += 1,
                    'O' => o += 1,
                    'T' => {
                        x += 1;
                        o += 1;
                    }
                    _ => {}
                }
            }

            ret = check_result(x, o);

            if !matches!(ret, GameResult::Draw) {
                break;
            }

            x = 0;
            o = 0;

            // Check diagonal
            for k in 0..4 {
                match board[k][k] {
                    'X' => x += 1,
                    'O' => o += 1,
                    'T' => {
                        x += 1;
                        o += 1;
                    }
                    _ => {}
                }
            }

            ret = check_result(x, o);

            if !matches!(ret, GameResult::Draw) {
                break;
            }

            x = 0;
            o = 0;

            // Check anti-diagonal
            for k in 0..4 {
                match board[k][3 - k] {
                    'X' => x += 1,
                    'O' => o += 1,
                    'T' => {
                        x += 1;
                        o += 1;
                    }
                    _ => {}
                }
            }

            ret = check_result(x, o);

            if !matches!(ret, GameResult::Draw) {
                break;
            }
        }

        if matches!(ret, GameResult::Draw) && board.iter().flatten().any(|&c| c == '.') {
            writeln!(out, "Case #{i}: Game has not completed").unwrap();
            continue;
        }

        writeln!(
            out,
            "Case #{i}: {}",
            match ret {
                GameResult::XWon => "X won",
                GameResult::OWon => "O won",
                GameResult::Draw => "Draw",
            }
        )
        .unwrap();
    }
}
