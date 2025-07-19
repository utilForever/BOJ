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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut bombs = vec![vec![0; m + 2]; n + 2];

    for i in 1..=n {
        for j in 1..=m {
            bombs[i][j] = scan.token::<i64>();
        }
    }

    let dy = [0, 0, -1, 1];
    let dx = [-1, 1, 0, 0];
    let mut ret = 0;

    for i in 1..=n {
        for j in 1..=m {
            let mut check = true;

            for k in 0..4 {
                let y_next = i as isize + dy[k];
                let x_next = j as isize + dx[k];

                if bombs[y_next as usize][x_next as usize] >= bombs[i][j] {
                    check = false;
                    break;
                }
            }

            if check {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
