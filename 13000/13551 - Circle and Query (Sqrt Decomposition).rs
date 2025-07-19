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
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

const MAX: i64 = 1_000_000;
const BUCKET: i64 = 32_768;
const SIZE: usize = MAX as usize / BUCKET as usize + 2;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut cells = vec![Vec::new(); SIZE * SIZE];
    let mut cnt = vec![0; SIZE * SIZE];

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let bucket_x = (x / BUCKET) as usize;
        let bucket_y = (y / BUCKET) as usize;
        let idx = bucket_y * SIZE + bucket_x;

        cells[idx].push(Point::new(x, y));
        cnt[idx] += 1;
    }

    let mut prefix_sum = vec![vec![0; SIZE + 1]; SIZE + 1];

    for y in 0..SIZE {
        let mut row = 0;

        for x in 0..SIZE {
            row += cnt[y * SIZE + x];
            prefix_sum[y + 1][x + 1] = prefix_sum[y][x + 1] + row;
        }
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let (cx, cy, r) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let bucket_x0 = ((cx - r) / BUCKET).max(0);
        let bucket_y0 = ((cy - r) / BUCKET).max(0);
        let bucket_x1 = (cx + r).min(MAX) / BUCKET;
        let bucket_y1 = (cy + r).min(MAX) / BUCKET;

        let inside = |x: i64, y: i64| -> bool {
            let x0 = x * BUCKET;
            let y0 = y * BUCKET;
            let dx = (cx - x0).abs().max((cx - (x0 + BUCKET)).abs());
            let dy = (cy - y0).abs().max((cy - (y0 + BUCKET)).abs());

            dx * dx + dy * dy <= r * r
        };

        let outside = |x: i64, y: i64| -> bool {
            let x0 = x * BUCKET;
            let y0 = y * BUCKET;
            let x1 = x0 + BUCKET;
            let y1 = y0 + BUCKET;

            let dx = if cx < x0 {
                x0 - cx
            } else if cx > x1 {
                cx - x1
            } else {
                0
            };
            let dy = if cy < y0 {
                y0 - cy
            } else if cy > y1 {
                cy - y1
            } else {
                0
            };

            dx * dx + dy * dy > r * r
        };

        let mut ret = 0;

        for y in bucket_y0..=bucket_y1 {
            for x in bucket_x0..=bucket_x1 {
                if outside(x, y) {
                    continue;
                }

                let idx = (y as usize) * SIZE + (x as usize);

                if inside(x, y) {
                    ret += cnt[idx];
                } else {
                    for &p in cells[idx].iter() {
                        let dx = p.x - cx;
                        let dy = p.y - cy;

                        if dx * dx + dy * dy <= r * r {
                            ret += 1;
                        }
                    }
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
