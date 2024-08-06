use io::Write;
use std::{collections::HashMap, io, str};

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

    let (_, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut indexes = vec![0; m];

    for i in 0..m {
        indexes[i] = scan.token::<i64>();
    }

    let moves = scan.token::<String>();
    let moves = moves.chars().collect::<Vec<char>>();
    let mut points = HashMap::new();
    let mut path = Vec::new();
    
    points.insert(0, 0);
    let mut initial_point = HashMap::new();
    initial_point.insert(0, 0);
    path.push(initial_point);

    for i in 0..m {
        let direction = if moves[i] == '+' {
            1
        } else if moves[i] == '-' {
            -1
        } else {
            0
        };

        if direction == 0 {
            break;
        }

        if points.contains_key(&indexes[i]) {
            *points.get_mut(&indexes[i]).unwrap() += direction;
        } else {
            points.insert(indexes[i], direction);
        }

        if points.get(&indexes[i]).unwrap() == &0 {
            points.remove(&indexes[i]);
        }

        for val in path.clone() {
            if val == points {
                writeln!(out, "0").unwrap();
                return;
            }
        }

        path.push(points.clone());
    }

    writeln!(out, "1").unwrap();
}
