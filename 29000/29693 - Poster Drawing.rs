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

    let (y, x) = (scan.token::<usize>(), scan.token::<usize>());
    let mut paper = vec![vec![' '; x]; y];

    for i in 0..y {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            paper[i][j] = c;
        }
    }

    let mut ret = false;

    if x % 2 == 1 {
        for i in 0..y {
            if paper[i][x / 2 - 1] == 'X' && paper[i][x / 2] == 'X' && paper[i][x / 2 + 1] == 'X' {
                ret = true;

                paper[i][x / 2 - 1] = 'W';
                paper[i][x / 2] = 'Y';
                paper[i][x / 2 + 1] = 'W';

                break;
            }
        }
    } else {
        for i in 0..y {
            if paper[i][x / 2 - 2] == 'X'
                && paper[i][x / 2 - 1] == 'X'
                && paper[i][x / 2] == 'X'
                && paper[i][x / 2 + 1] == 'X'
            {
                ret = true;

                paper[i][x / 2 - 2] = 'W';
                paper[i][x / 2 - 1] = 'Y';
                paper[i][x / 2] = 'Y';
                paper[i][x / 2 + 1] = 'W';

                break;
            }
        }
    }

    if ret {
        writeln!(out, "YES").unwrap();

        for i in 0..y {
            for j in 0..x {
                write!(
                    out,
                    "{}",
                    if paper[i][j] == 'X' { 'B' } else { paper[i][j] }
                )
                .unwrap();
            }

            writeln!(out).unwrap();
        }
    } else {
        writeln!(out, "NO").unwrap();
    }
}
