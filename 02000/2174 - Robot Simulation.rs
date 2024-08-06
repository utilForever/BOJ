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

    let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut robots = vec![(0, 0, ' '); n + 1];

    for i in 1..=n {
        let (x, y, dir) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<String>(),
        );

        robots[i] = (x, y, dir.chars().next().unwrap());
    }

    for _ in 0..m {
        let (idx, command, count) = (
            scan.token::<usize>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );
        let command = command.chars().next().unwrap();

        for _ in 0..count {
            match command {
                'L' => {
                    robots[idx].2 = match robots[idx].2 {
                        'N' => 'W',
                        'W' => 'S',
                        'S' => 'E',
                        'E' => 'N',
                        _ => unreachable!(),
                    }
                }
                'R' => {
                    robots[idx].2 = match robots[idx].2 {
                        'N' => 'E',
                        'E' => 'S',
                        'S' => 'W',
                        'W' => 'N',
                        _ => unreachable!(),
                    }
                }
                'F' => {
                    match robots[idx].2 {
                        'N' => robots[idx].1 += 1,
                        'W' => robots[idx].0 -= 1,
                        'S' => robots[idx].1 -= 1,
                        'E' => robots[idx].0 += 1,
                        _ => unreachable!(),
                    }

                    if robots[idx].0 <= 0
                        || robots[idx].0 >= a + 1
                        || robots[idx].1 <= 0
                        || robots[idx].1 >= b + 1
                    {
                        writeln!(out, "Robot {} crashes into the wall", idx).unwrap();
                        return;
                    }

                    for robot in robots.iter().enumerate() {
                        if robot.0 == idx {
                            continue;
                        }

                        if robot.1 .0 == robots[idx].0 && robot.1 .1 == robots[idx].1 {
                            writeln!(out, "Robot {} crashes into robot {}", idx, robot.0).unwrap();
                            return;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    writeln!(out, "OK").unwrap();
}
