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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut room = vec![vec![0; m]; n];

    // Direction => 0: North, 1: East, 2: South, 3: West
    let (mut pos, mut direction) = (
        (scan.token::<usize>(), scan.token::<usize>()),
        scan.token::<i64>(),
    );

    for i in 0..n {
        for j in 0..m {
            room[i][j] = scan.token::<i64>();
        }
    }

    let mut ret = 0;

    loop {
        // Step 1: Clean the cell if it is not cleaned
        if room[pos.0][pos.1] == 0 {
            room[pos.0][pos.1] = 2;
            ret += 1;
        }

        let cell_north = if pos.0 > 0 { room[pos.0 - 1][pos.1] } else { 1 };
        let cell_east = if pos.1 < m - 1 {
            room[pos.0][pos.1 + 1]
        } else {
            1
        };
        let cell_south = if pos.0 < n - 1 {
            room[pos.0 + 1][pos.1]
        } else {
            1
        };
        let cell_west = if pos.1 > 0 { room[pos.0][pos.1 - 1] } else { 1 };

        if cell_north != 0 && cell_east != 0 && cell_south != 0 && cell_west != 0 {
            // Step 2: If all adjacent cells are cleaned, move to back cell
            let cell_back = match direction {
                0 => cell_south,
                1 => cell_west,
                2 => cell_north,
                3 => cell_east,
                _ => unreachable!("Invalid direction"),
            };

            if cell_back == 1 {
                break;
            } else {
                match direction {
                    0 => pos.0 += 1,
                    1 => pos.1 -= 1,
                    2 => pos.0 -= 1,
                    3 => pos.1 += 1,
                    _ => unreachable!("Invalid direction"),
                }
            }
        } else {
            // Step 3: If there is a cell that is not cleaned, turn counterclockwise and move to the next cell
            for i in 1..=4 {
                let direction_next = (direction + 4 - i) % 4;
                let cell_next = match direction_next {
                    0 => cell_north,
                    1 => cell_east,
                    2 => cell_south,
                    3 => cell_west,
                    _ => unreachable!("Invalid direction"),
                };

                if cell_next == 0 {
                    direction = direction_next;
                    match direction {
                        0 => pos.0 -= 1,
                        1 => pos.1 += 1,
                        2 => pos.0 += 1,
                        3 => pos.1 -= 1,
                        _ => unreachable!("Invalid direction"),
                    }

                    break;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
