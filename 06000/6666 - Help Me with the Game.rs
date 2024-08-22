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

    let _ = scan.token::<String>();
    let mut blacks = vec![Vec::new(); 6];
    let mut whites = vec![Vec::new(); 6];

    for i in (1..=8).rev() {
        let board = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut column = b'a';
        let mut cnt = 2;

        while cnt < board.len() {
            if board[cnt] == ':' || board[cnt] == '.' {
                column += 1;
                cnt += 4;
                continue;
            }

            let is_black = if board[cnt].is_ascii_lowercase() {
                true
            } else {
                false
            };
            let mut info = String::new();

            if board[cnt] != 'p' && board[cnt] != 'P' {
                info.push(board[cnt].to_ascii_uppercase());
            }
            info.push(column as char);
            info.push(i.to_string().chars().next().unwrap());

            if is_black {
                match board[cnt] {
                    'k' => blacks[0].push(info),
                    'q' => blacks[1].push(info),
                    'r' => blacks[2].push(info),
                    'b' => blacks[3].push(info),
                    'n' => blacks[4].push(info),
                    _ => blacks[5].push(info),
                }
            } else {
                match board[cnt] {
                    'K' => whites[0].push(info),
                    'Q' => whites[1].push(info),
                    'R' => whites[2].push(info),
                    'B' => whites[3].push(info),
                    'N' => whites[4].push(info),
                    _ => whites[5].push(info),
                }
            }

            column += 1;
            cnt += 4;
        }

        let _ = scan.token::<String>();
    }

    for i in 0..6 {
        blacks[i].sort_by(|a, b| {
            let chars_a = a.chars().collect::<Vec<_>>();
            let chars_b = b.chars().collect::<Vec<_>>();

            let (row_a, col_a) = (chars_a[chars_a.len() - 1], chars_a[chars_a.len() - 2]);
            let (row_b, col_b) = (chars_b[chars_b.len() - 1], chars_b[chars_b.len() - 2]);

            if row_a == row_b {
                col_a.cmp(&col_b)
            } else {
                row_b.cmp(&row_a)
            }
        });

        whites[i].sort_by(|a, b| {
            let chars_a = a.chars().collect::<Vec<_>>();
            let chars_b = b.chars().collect::<Vec<_>>();

            let (row_a, col_a) = (chars_a[chars_a.len() - 1], chars_a[chars_a.len() - 2]);
            let (row_b, col_b) = (chars_b[chars_b.len() - 1], chars_b[chars_b.len() - 2]);

            if row_a == row_b {
                col_a.cmp(&col_b)
            } else {
                row_a.cmp(&row_b)
            }
        });
    }

    write!(out, "White: ").unwrap();

    for i in 0..6 {
        for (idx, val) in whites[i].iter().enumerate() {
            if i == 5 && idx == whites[i].len() - 1 {
                writeln!(out, "{val}").unwrap();
            } else {
                write!(out, "{val},").unwrap();
            }
        }
    }

    write!(out, "Black: ").unwrap();

    for i in 0..6 {
        for (idx, val) in blacks[i].iter().enumerate() {
            if i == 5 && idx == blacks[i].len() - 1 {
                writeln!(out, "{val}").unwrap();
            } else {
                write!(out, "{val},").unwrap();
            }
        }
    }
}
