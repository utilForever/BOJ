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

enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, pos) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ret = vec![vec![0; n]; n];
    let mut cnt = 0;
    let mut x = n / 2;
    let mut y = n / 2;
    let mut direction = Direction::North;
    let mut offset = 1;

    ret[y][x] = 1;
    cnt += 1;

    while cnt < n * n {
        match direction {
            Direction::North => {
                for _ in 0..offset {
                    y -= 1;
                    cnt += 1;
                    ret[y][x] = cnt;

                    if cnt == n * n {
                        break;
                    }
                }
            }
            Direction::East => {
                for _ in 0..offset {
                    x += 1;
                    cnt += 1;
                    ret[y][x] = cnt;
                }

                offset += 1;
            }
            Direction::South => {
                for _ in 0..offset {
                    y += 1;
                    cnt += 1;
                    ret[y][x] = cnt;
                }
            }
            Direction::West => {
                for _ in 0..offset {
                    x -= 1;
                    cnt += 1;
                    ret[y][x] = cnt;
                }

                offset += 1;
            }
        }

        direction = match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }

    for i in 0..n {
        for j in 0..n {
            if ret[i][j] == pos {
                writeln!(out, "{} {}", i + 1, j + 1).unwrap();
                break;
            }
        }
    }
}
