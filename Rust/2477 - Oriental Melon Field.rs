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

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    East,
    West,
    South,
    North,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<i64>();
    let mut area = vec![(Direction::East, 0); 6];

    for i in 0..6 {
        let (dir, len) = (scan.token::<i64>(), scan.token::<i64>());

        area[i] = match dir {
            1 => (Direction::East, len),
            2 => (Direction::West, len),
            3 => (Direction::South, len),
            4 => (Direction::North, len),
            _ => unreachable!(),
        };
    }

    let mut area_large = 0;
    let mut area_small = 0;

    for i in 0..6 {
        if area[i].0 == area[(i + 2) % 6].0 && area[(i + 1) % 6].0 == area[(i + 3) % 6].0 {
            area_large =
                (area[i].1 + area[(i + 2) % 6].1) * (area[(i + 1) % 6].1 + area[(i + 3) % 6].1);
            area_small = area[(i + 1) % 6].1 * area[(i + 2) % 6].1;
        }
    }

    writeln!(out, "{}", k * (area_large - area_small)).unwrap();
}
