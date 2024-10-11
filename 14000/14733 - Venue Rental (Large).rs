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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut rectangles = vec![(0, 0, 0, 0); n];
    let mut coords_x = vec![0; 2 * n];
    let mut coords_y = vec![0; 2 * n];

    for i in 0..n {
        rectangles[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        coords_x[2 * i] = rectangles[i].0;
        coords_x[2 * i + 1] = rectangles[i].2;
        coords_y[2 * i] = rectangles[i].1;
        coords_y[2 * i + 1] = rectangles[i].3;
    }

    coords_x.sort_unstable();
    coords_x.dedup();
    coords_y.sort_unstable();
    coords_y.dedup();

    let mapping_x = |x: i64| -> usize {
        match coords_x.binary_search(&x) {
            Ok(idx) => idx,
            Err(_) => unreachable!(),
        }
    };
    let mapping_y = |y: i64| -> usize {
        match coords_y.binary_search(&y) {
            Ok(idx) => idx,
            Err(_) => unreachable!(),
        }
    };

    let len_x = coords_x.len();
    let len_y = coords_y.len();
    let mut area = vec![0; len_x * len_y];

    for &(x1, y1, x2, y2) in rectangles.iter() {
        let (idx_x1, idx_x2) = (mapping_x(x1), mapping_x(x2));
        let (idx_y1, idx_y2) = (mapping_y(y1), mapping_y(y2));

        area[idx_x1 * len_y + idx_y1] += 1;
        area[idx_x2 * len_y + idx_y1] -= 1;
        area[idx_x1 * len_y + idx_y2] -= 1;
        area[idx_x2 * len_y + idx_y2] += 1;
    }

    // Calculate the prefix sum (Horizontal)
    for x in 0..len_x {
        for y in 1..len_y {
            area[x * len_y + y] += area[x * len_y + y - 1];
        }
    }

    // Calculate the prefix sum (Vertical)
    for y in 0..len_y {
        for x in 1..len_x {
            area[x * len_y + y] += area[(x - 1) * len_y + y];
        }
    }

    // Calculate the area
    let mut ret: i64 = 0;

    for x in 0..len_x - 1 {
        for y in 0..len_y - 1 {
            let count = area[x * len_y + y];

            if count > 0 {
                let delta_x = (coords_x[x + 1] - coords_x[x]) as i64;
                let delta_y = (coords_y[y + 1] - coords_y[y]) as i64;

                ret += delta_x * delta_y;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
