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

fn find_kth(buckets: &Vec<i64>, k: i64) -> usize {
    let mut acc = 0;

    for (i, &val) in buckets.iter().enumerate() {
        acc += val;

        if acc >= k {
            return i;
        }
    }

    unreachable!("kth element must exist");
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

    let stride = w + 1;
    let mut prefix_sum = vec![0; (h + 1) * stride];
    let mut s_max = 0;

    for y in 1..=h {
        let row_prev = (y - 1) * stride;
        let row_curr = y * stride;

        for x in 1..=w {
            let s = scan.token::<i64>();
            let idx = row_curr + x;

            prefix_sum[idx] = prefix_sum[row_prev + x] + prefix_sum[row_curr + x - 1]
                - prefix_sum[row_prev + x - 1]
                + s;
            s_max = s_max.max(s);
        }
    }

    let bucket_max = (s_max as usize) * 1000;
    let mut buckets = vec![0; bucket_max + 1];

    let mut strip = vec![0; stride];
    let mut cnt = 0;

    for y1 in 0..h {
        let row1 = y1 * stride;

        for y2 in y1 + 1..=h {
            let height = y2 - y1;
            let mut w_min = (a + height - 1) / height;
            let mut w_max = b / height;

            w_min = w_min.max(1);

            if w_min > w_max || w_min > w {
                continue;
            }

            w_max = w_max.min(w);

            let row2 = y2 * stride;

            for x in 0..=w {
                strip[x] = prefix_sum[row2 + x] - prefix_sum[row1 + x];
            }

            for x1 in 0..w {
                let start = x1 + w_min;

                if start > w {
                    break;
                }

                let end = (x1 + w_max).min(w);
                let base = strip[x1];

                for x2 in start..=end {
                    let width = x2 - x1;
                    let area = (height * width) as i64;
                    let sum = strip[x2] - base;
                    let idx = ((sum * 1000) / area) as usize;

                    buckets[idx] += 1;
                    cnt += 1;
                }
            }
        }
    }

    let ret = if cnt % 2 == 1 {
        find_kth(&buckets, cnt / 2 + 1)
    } else {
        let i = find_kth(&buckets, cnt / 2);
        let j = find_kth(&buckets, cnt / 2 + 1);
        (i + j + 1) / 2
    };

    writeln!(out, "{:.3}", ret as f64 / 1000.0).unwrap();
}
