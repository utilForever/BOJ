use io::Write;
use std::{collections::BTreeSet, io, str};

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

    let (n, q, t) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut ufo_x = vec![BTreeSet::new(); 200_001];
    let mut ufo_y = vec![BTreeSet::new(); 200_001];
    let mut sweeping_x = vec![Vec::new(); 100_001];
    let mut sweeping_y = vec![Vec::new(); 100_001];
    let mut ret_x = vec![0; 100_001];
    let mut ret_y = vec![0; 100_001];

    for _ in 0..n {
        let (x, y, dx, dy) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if dx == 0 {
            ret_y[x as usize] += 1;
        } else {
            ufo_y[(dx + 100_000) as usize].insert(x % dx);
            sweeping_y[x as usize].push((dx, 1));

            if x + dx * t >= 0 && x + dx * t <= 100_000 {
                sweeping_y[(x + dx * t) as usize].push((dx, -1));
            }
        }

        if dy == 0 {
            ret_x[y as usize] += 1;
        } else {
            ufo_x[(dy + 100_000) as usize].insert(y % dy);
            sweeping_x[y as usize].push((dy, 1));

            if y + dy * t >= 0 && y + dy * t <= 100_000 {
                sweeping_x[(y + dy * t) as usize].push((dy, -1));
            }
        }
    }

    for i in 0..=100_000 {
        sweeping_x[i].sort();
        sweeping_y[i].sort();
    }

    for d in (-100_000..=100_000).rev() {
        if d == 0 {
            continue;
        }

        for &set_y in ufo_y[(d + 100_000) as usize].iter() {
            let mut idx = if d > 0 { set_y } else { set_y + (100000 - set_y) / d * d };
            let mut cnt = 0;

            while idx >= 0 && idx <= 100_000 {
                while !sweeping_y[idx as usize].is_empty() && sweeping_y[idx as usize].last().unwrap().0 == d {
                    cnt += sweeping_y[idx as usize].pop().unwrap().1;
                }

                ret_y[idx as usize] += cnt;
                idx += d;
            }
        }

        for &set_x in ufo_x[(d + 100_000) as usize].iter() {
            let mut idx = if d > 0 { set_x } else { set_x + (100000 - set_x) / d * d };
            let mut cnt = 0;

            while idx >= 0 && idx <= 100_000 {
                while !sweeping_x[idx as usize].is_empty() && sweeping_x[idx as usize].last().unwrap().0 == d {
                    cnt += sweeping_x[idx as usize].pop().unwrap().1;
                }

                ret_x[idx as usize] += cnt;
                idx += d;
            }
        }
    }

    for _ in 0..q {
        let (op, axis) = (scan.token::<i64>(), scan.token::<usize>());
        writeln!(out, "{}", if op == 1 { ret_x[axis] } else { ret_y[axis] }).unwrap();
    }
}
