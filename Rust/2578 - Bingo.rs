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

    let mut bingo = [[0; 5]; 5];

    for i in 0..5 {
        for j in 0..5 {
            bingo[i][j] = scan.token::<i64>();
        }
    }

    // Can 3 bingos be made?
    let is_bingo = |bingo: &[[i64; 5]; 5]| -> bool {
        let mut cnt_bingo = 0;

        // Check rows
        for i in 0..5 {
            let mut flag = true;

            for j in 0..5 {
                if bingo[i][j] != 0 {
                    flag = false;
                    break;
                }
            }

            if flag {
                cnt_bingo += 1;
            }
        }

        // Check columns
        for i in 0..5 {
            let mut flag = true;

            for j in 0..5 {
                if bingo[j][i] != 0 {
                    flag = false;
                    break;
                }
            }

            if flag {
                cnt_bingo += 1;
            }
        }

        // Check diagonals
        let mut flag = true;

        for i in 0..5 {
            if bingo[i][i] != 0 {
                flag = false;
                break;
            }
        }

        if flag {
            cnt_bingo += 1;
        }

        let mut flag = true;

        for i in 0..5 {
            if bingo[i][4 - i] != 0 {
                flag = false;
                break;
            }
        }

        if flag {
            cnt_bingo += 1;
        }

        if cnt_bingo >= 3 {
            true
        } else {
            false
        }
    };

    let mut made_bingo = false;
    let mut cnt = 0;
    let mut ret = 0;

    for _ in 0..25 {
        let num = scan.token::<i64>();

        cnt += 1;

        if !made_bingo {
            for j in 0..5 {
                for k in 0..5 {
                    if bingo[j][k] == num {
                        bingo[j][k] = 0;
                    }
                }
            }

            if is_bingo(&bingo) {
                made_bingo = true;
                ret = cnt;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
