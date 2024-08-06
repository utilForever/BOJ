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

    let n = scan.token::<usize>();
    let mut sheeps = vec![(0, 0); n];

    for i in 0..n {
        sheeps[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let x_min = *sheeps.iter().map(|(x, _)| x).min().unwrap();
    let x_max = *sheeps.iter().map(|(x, _)| x).max().unwrap();
    let y_min = *sheeps.iter().map(|(_, y)| y).min().unwrap();
    let y_max = *sheeps.iter().map(|(_, y)| y).max().unwrap();
    let num_walls = (x_max - x_min + y_max - y_min + 2) * 2;

    let mut min = HashMap::new();
    let mut max = HashMap::new();

    for (x, y) in sheeps {
        *min.entry(x).or_insert(y) = y.min(*min.get(&x).unwrap_or(&y));
        *max.entry(x).or_insert(y) = y.max(*max.get(&x).unwrap_or(&y));
    }

    let mut area = (x_max - x_min + 1) * (y_max - y_min + 1);

    // Sweep 1
    {
        let mut min_vec = min.iter().collect::<Vec<_>>();
        min_vec.sort();

        let mut x = *min_vec[0].0;
        let mut y = *min_vec[0].1;

        for i in 1..min_vec.len() {
            area -= (*min_vec[i].0 - x) * (y - y_min);
            max.entry(*min_vec[i].0).and_modify(|v| *v = *v.max(&mut y));

            if *min_vec[i].1 == y_min {
                break;
            }

            x = *min_vec[i].0;
            y = y.min(*min_vec[i].1);
        }
    }

    // Sweep 2
    {
        let mut min_vec = min.iter().collect::<Vec<_>>();
        min_vec.sort();
        min_vec.reverse();

        let mut x = *min_vec[0].0;
        let mut y = *min_vec[0].1;

        for i in 1..min_vec.len() {
            area -= (x - *min_vec[i].0) * (y - y_min);
            max.entry(*min_vec[i].0).and_modify(|v| *v = *v.max(&mut y));

            if *min_vec[i].1 == y_min {
                break;
            }

            x = *min_vec[i].0;
            y = y.min(*min_vec[i].1);
        }
    }

    // Sweep 3
    {
        let mut max_vec = max.iter().collect::<Vec<_>>();
        max_vec.sort();

        let mut x = *max_vec[0].0;
        let mut y = *max_vec[0].1;

        for i in 1..max_vec.len() {
            area -= (*max_vec[i].0 - x) * (y_max - y);

            if *max_vec[i].1 == y_max {
                break;
            }

            x = *max_vec[i].0;
            y = y.max(*max_vec[i].1);
        }
    }

    // Sweep 4
    {
        let mut max_vec = max.iter().collect::<Vec<_>>();
        max_vec.sort();
        max_vec.reverse();

        let mut x = *max_vec[0].0;
        let mut y = *max_vec[0].1;

        for i in 1..max_vec.len() {
            area -= (x - *max_vec[i].0) * (y_max - y);

            if *max_vec[i].1 == y_max {
                break;
            }

            x = *max_vec[i].0;
            y = y.max(*max_vec[i].1);
        }
    }

    writeln!(out, "{num_walls} {area}").unwrap();
}
