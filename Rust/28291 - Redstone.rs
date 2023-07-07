use io::Write;
use std::{collections::VecDeque, io, str};

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

    let (w, h) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![-1; w]; h];
    let mut lamps = Vec::new();
    let mut queue = VecDeque::new();

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (b, x, y) = (
            scan.token::<String>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        match b.as_str() {
            "redstone_dust" => {
                map[y][x] = 0;
            }
            "redstone_block" => {
                map[y][x] = 16;
                queue.push_back((y, x, 16));
            }
            "redstone_lamp" => {
                map[y][x] = -2;
                lamps.push((y, x));
            }
            _ => unreachable!(),
        }
    }

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    while !queue.is_empty() {
        let (y, x, power) = queue.pop_front().unwrap();

        for i in 0..4 {
            let (ny, nx) = (y as i64 + dy[i], x as i64 + dx[i]);

            if ny < 0 || nx < 0 || ny >= h as i64 || nx >= w as i64 {
                continue;
            }

            if map[ny as usize][nx as usize] == -2 {
                map[ny as usize][nx as usize] = power - 1;
                continue;
            }

            if map[ny as usize][nx as usize] == -1 || map[ny as usize][nx as usize] >= power - 1 {
                continue;
            }

            map[ny as usize][nx as usize] = power - 1;
            queue.push_back((ny as usize, nx as usize, power - 1));
        }
    }

    for lamp in lamps {
        let (y, x) = lamp;

        if map[y][x] <= 0 {
            writeln!(out, "failed").unwrap();
            return;
        }
    }

    writeln!(out, "success").unwrap();
}
