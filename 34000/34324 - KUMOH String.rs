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

const KUMOH: [char; 5] = ['K', 'U', 'M', 'O', 'H'];
const HOMUK: [char; 5] = ['H', 'O', 'M', 'U', 'K'];

fn calculate(grid: &Vec<Vec<char>>, mut i: i64, mut j: usize) -> i64 {
    let mut cnt_kumoh = 0;
    let mut cnt_homuk = 0;
    let mut s = vec![' '; 5];
    let mut idx = 0;

    while i >= 0 && j < 1000 {
        let row = &grid[i as usize];

        if j < row.len() {
            if idx < 5 {
                s[idx] = row[j];
                idx += 1;

                if idx == 5 {
                    if s == KUMOH {
                        cnt_kumoh += 1;
                    }

                    if s == HOMUK {
                        cnt_homuk += 1;
                    }
                }
            } else {
                s.rotate_left(1);
                s[4] = row[j];

                if s == KUMOH {
                    cnt_kumoh += 1;
                }

                if s == HOMUK {
                    cnt_homuk += 1;
                }
            }
        }

        i -= 1;
        j += 1;
    }

    cnt_kumoh.max(cnt_homuk)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut grid = vec![Vec::new(); n];

    for i in 0..n {
        grid[i] = scan.token::<String>().chars().collect();
    }

    let mut ret = 0;

    for i in 0..n {
        ret += calculate(&grid, i as i64, 0);
    }

    for j in 1..1000 {
        ret += calculate(&grid, n as i64 - 1, j);
    }

    writeln!(out, "{ret}").unwrap();
}
