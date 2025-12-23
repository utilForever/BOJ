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

#[derive(Clone, Copy)]
struct BoundingBox {
    y_min: i64,
    y_max: i64,
    x_min: i64,
    x_max: i64,
}

impl BoundingBox {
    fn new(y: usize, x: usize) -> Self {
        Self {
            y_min: (y as i64) + 1,
            y_max: 0,
            x_min: (x as i64) + 1,
            x_max: 0,
        }
    }

    fn update(&mut self, y: i64, x: i64) {
        self.y_min = self.y_min.min(y);
        self.y_max = self.y_max.max(y);
        self.x_min = self.x_min.min(x);
        self.x_max = self.x_max.max(x);
    }

    fn exists(&self) -> bool {
        self.y_max != 0
    }
}

fn build_prefix_2d(prefix_sum: &mut Vec<i64>, n: usize, m: usize) {
    for i in 1..=n {
        let row = i * (m + 1);
        let base = (i - 1) * (m + 1);

        for j in 1..=m {
            let idx = row + j;

            prefix_sum[idx] +=
                prefix_sum[base + j] + prefix_sum[row + j - 1] - prefix_sum[base + j - 1];
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, d) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut grid = vec![vec![0; m]; n];
    let mut bbox = vec![BoundingBox::new(n, m); n * m + 1];

    for i in 1..=n {
        for j in 1..=m {
            let id_room = scan.token::<usize>();
            grid[i - 1][j - 1] = id_room;
            bbox[id_room].update(i as i64, j as i64);
        }
    }

    let mut spells = vec![0; n * m + 1];

    for i in 0..n {
        for j in 0..m {
            let spell = scan.token::<i64>();
            spells[grid[i][j]] = spell;
        }
    }

    let mut sum_top = vec![0; n + 1];
    let mut sum_bottom = vec![0; n + 1];
    let mut sum_left = vec![0; m + 1];
    let mut sum_right = vec![0; m + 1];
    let mut prefix_sum_tl = vec![0; (n + 1) * (m + 1)];
    let mut prefix_sum_tr = vec![0; (n + 1) * (m + 1)];
    let mut prefix_sum_bl = vec![0; (n + 1) * (m + 1)];
    let mut prefix_sum_br = vec![0; (n + 1) * (m + 1)];
    let mut sum = 0;

    for id in 1..=n * m {
        if !bbox[id].exists() {
            continue;
        }

        let spell = spells[id];
        sum += spell;

        let top = bbox[id].y_min as usize;
        let bottom = bbox[id].y_max as usize;
        let left = bbox[id].x_min as usize;
        let right = bbox[id].x_max as usize;

        sum_top[n + 1 - top] += spell;
        sum_bottom[bottom] += spell;
        sum_left[m + 1 - left] += spell;
        sum_right[right] += spell;

        prefix_sum_tl[(n + 1 - top) * (m + 1) + (m + 1 - left)] += spell;
        prefix_sum_tr[(n + 1 - top) * (m + 1) + right] += spell;
        prefix_sum_bl[bottom * (m + 1) + (m + 1 - left)] += spell;
        prefix_sum_br[bottom * (m + 1) + right] += spell;
    }

    for i in 1..=n {
        sum_top[i] += sum_top[i - 1];
        sum_bottom[i] += sum_bottom[i - 1];
    }

    for i in 1..=m {
        sum_left[i] += sum_left[i - 1];
        sum_right[i] += sum_right[i - 1];
    }

    build_prefix_2d(&mut prefix_sum_tl, n, m);
    build_prefix_2d(&mut prefix_sum_tr, n, m);
    build_prefix_2d(&mut prefix_sum_bl, n, m);
    build_prefix_2d(&mut prefix_sum_br, n, m);

    for _ in 0..d {
        let (r1, c1, r2, c2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let y1 = r1 - 1;
        let x1 = c1 - 1;
        let y2 = n - r2;
        let x2 = m - c2;

        let mut ret = sum;

        ret -= sum_top[y2];
        ret -= sum_bottom[y1];
        ret -= sum_left[x2];
        ret -= sum_right[x1];

        ret += prefix_sum_tl[y2 * (m + 1) + x2];
        ret += prefix_sum_tr[y2 * (m + 1) + x1];
        ret += prefix_sum_bl[y1 * (m + 1) + x2];
        ret += prefix_sum_br[y1 * (m + 1) + x1];

        writeln!(out, "{ret}").unwrap();
    }
}
