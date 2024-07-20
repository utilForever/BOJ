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

    let mut ret = vec![vec![' '; n]; n];
    let mut is_touch_bomb = false;

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            if c == 'x' && board[i][j] == '*' {
                is_touch_bomb = true;
            }

            ret[i][j] = if c == 'x' {
                let mut count = 0;

                for k in (i.saturating_sub(1))..=(i + 1).min(n - 1) {
                    for l in (j.saturating_sub(1))..=(j + 1).min(n - 1) {
                        if board[k][l] == '*' {
                            count += 1;
                        }
                    }
                }

                count.to_string().chars().next().unwrap()
            } else {
                c
            };
        }
    }

    if is_touch_bomb {
        for i in 0..n {
            for j in 0..n {
                if board[i][j] == '*' {
                    ret[i][j] = '*';
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
