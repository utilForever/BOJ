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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let commands = scan.token::<String>().chars().collect::<Vec<_>>();
        let (mut y, mut x) = (0, 0);
        let (mut x_min, mut x_max, mut y_min, mut y_max) = (0, 0, 0, 0);
        let mut direction = Direction::North;

        for c in commands {
            match c {
                'F' => match direction {
                    Direction::North => y += 1,
                    Direction::East => x += 1,
                    Direction::South => y -= 1,
                    Direction::West => x -= 1,
                },
                'B' => match direction {
                    Direction::North => y -= 1,
                    Direction::East => x -= 1,
                    Direction::South => y += 1,
                    Direction::West => x += 1,
                },
                'L' => {
                    direction = match direction {
                        Direction::North => Direction::West,
                        Direction::East => Direction::North,
                        Direction::South => Direction::East,
                        Direction::West => Direction::South,
                    }
                }
                'R' => {
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    }
                }
                _ => (),
            }

            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }

        writeln!(out, "{}", (x_max - x_min) * (y_max - y_min)).unwrap();
    }
}
