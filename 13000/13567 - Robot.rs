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

#[derive(Clone)]
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

    let (m, n) = (scan.token::<i64>(), scan.token::<i64>());
    let mut pos = (0, 0);
    let mut direction = Direction::East;
    let mut is_valid = true;

    for _ in 0..n {
        let command = scan.token::<String>();

        if command == "TURN" {
            let dir = scan.token::<i64>();

            direction = if dir == 0 {
                match direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                }
            } else {
                match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            }
        } else {
            let d = scan.token::<i64>();
            let pos_next = match direction {
                Direction::North => (pos.0, pos.1 + d),
                Direction::East => (pos.0 + d, pos.1),
                Direction::South => (pos.0, pos.1 - d),
                Direction::West => (pos.0 - d, pos.1),
            };

            if pos_next.0 < 0 || pos_next.0 > m || pos_next.1 < 0 || pos_next.1 > m {
                is_valid = false;
            }

            pos = pos_next;
        }
    }

    if is_valid {
        writeln!(out, "{} {}", pos.0, pos.1).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
