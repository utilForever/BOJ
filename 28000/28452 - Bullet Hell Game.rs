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

const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let (xc, yc) = (scan.token::<usize>(), scan.token::<usize>());
    let k = scan.token::<usize>();
    let mut bullets = vec![(0, 0); k];

    for i in 0..k {
        bullets[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut queue = VecDeque::new();
    let mut dist_bullets = vec![vec![i64::MAX; m]; n];

    for &(x, y) in bullets.iter() {
        if dist_bullets[x][y] == i64::MAX {
            dist_bullets[x][y] = 0;
            queue.push_back((x, y));
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let dist_curr = dist_bullets[x][y];

        for &(x_curr, y_curr) in &DIRECTIONS {
            let x_next = x as i64 + x_curr;
            let y_next = y as i64 + y_curr;

            if x_next < 0 || x_next >= n as i64 || y_next < 0 || y_next >= m as i64 {
                continue;
            }

            let x_next = x_next as usize;
            let y_next = y_next as usize;

            if dist_bullets[x_next][y_next] > dist_curr + 1 {
                dist_bullets[x_next][y_next] = dist_curr + 1;
                queue.push_back((x_next, y_next));
            }
        }
    }

    let mut ret = false;

    'outer: for x in 0..n {
        let dx = (xc as i64 - x as i64).abs();

        for y in 0..m {
            let dy = (yc as i64 - y as i64).abs();
            let dist_player = dx.max(dy);
            let dist_bullet = dist_bullets[x][y];

            if dist_player <= t && dist_bullet > t {
                ret = true;
                break 'outer;
            }
        }
    }

    writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
}
