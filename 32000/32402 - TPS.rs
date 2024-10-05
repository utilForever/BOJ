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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut pos = (0, -1);
    let mut direction = Direction::Up;

    for _ in 0..n {
        let input = scan.token::<String>();

        match input.as_str() {
            "W" => match direction {
                Direction::Up => {
                    pos.1 += 1;
                }
                Direction::Down => {
                    pos.1 -= 1;
                }
                Direction::Left => {
                    pos.0 -= 1;
                }
                Direction::Right => {
                    pos.0 += 1;
                }
            },
            "A" => match direction {
                Direction::Up => {
                    pos.0 -= 1;
                }
                Direction::Down => {
                    pos.0 += 1;
                }
                Direction::Left => {
                    pos.1 -= 1;
                }
                Direction::Right => {
                    pos.1 += 1;
                }
            },
            "S" => match direction {
                Direction::Up => {
                    pos.1 -= 1;
                }
                Direction::Down => {
                    pos.1 += 1;
                }
                Direction::Left => {
                    pos.0 += 1;
                }
                Direction::Right => {
                    pos.0 -= 1;
                }
            },
            "D" => match direction {
                Direction::Up => {
                    pos.0 += 1;
                }
                Direction::Down => {
                    pos.0 -= 1;
                }
                Direction::Left => {
                    pos.1 += 1;
                }
                Direction::Right => {
                    pos.1 -= 1;
                }
            },
            "MR" => {
                pos = match direction {
                    Direction::Up => (pos.0 - 1, pos.1 + 1),
                    Direction::Down => (pos.0 + 1, pos.1 - 1),
                    Direction::Left => (pos.0 - 1, pos.1 - 1),
                    Direction::Right => (pos.0 + 1, pos.1 + 1),
                };

                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
            "ML" => {
                pos = match direction {
                    Direction::Up => (pos.0 + 1, pos.1 + 1),
                    Direction::Down => (pos.0 - 1, pos.1 - 1),
                    Direction::Left => (pos.0 - 1, pos.1 + 1),
                    Direction::Right => (pos.0 + 1, pos.1 - 1),
                };

                direction = match direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
            }
            _ => unreachable!(),
        }

        let pos_player = match direction {
            Direction::Up => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0, pos.1 - 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        };

        writeln!(out, "{} {} {} {}", pos_player.0, pos_player.1, pos.0, pos.1).unwrap();
    }
}
