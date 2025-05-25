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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<i64>();
        let mut ret = 6 * n;

        for a in 1..=(n as f64).cbrt().ceil() as i64 {
            for b in a..=((n as f64 / a as f64).sqrt().ceil() as i64) {
                let blocks_complete = n / (a * b);
                let blocks_remain = n % (a * b);

                if blocks_remain == 0 {
                    let surface = 2 * (a * b + b * blocks_complete + a * blocks_complete);
                    ret = ret.min(surface);
                    continue;
                }

                let touch_complete_side = blocks_complete * (2 * a * b - a - b);
                let touch_complete_bottom = if blocks_complete >= 2 {
                    (blocks_complete - 1) * a * b
                } else {
                    0
                };

                for c in 1..=a.min(blocks_remain) {
                    let complete = blocks_remain / c;
                    let remain = blocks_remain % c;

                    let touch_horizontal =
                        complete * (c - 1) + if remain > 0 { remain - 1 } else { 0 };
                    let touch_vertical = if complete > 0 {
                        (complete - 1) * c + if remain > 0 { remain } else { 0 }
                    } else {
                        0
                    };

                    let touch_partial_side = touch_horizontal + touch_vertical;
                    let touch_partial_bottom = if blocks_complete > 0 {
                        blocks_remain
                    } else {
                        0
                    };

                    let touch_total = touch_complete_side
                        + touch_complete_bottom
                        + touch_partial_side
                        + touch_partial_bottom;

                    let surface = 6 * n - 2 * touch_total;
                    ret = ret.min(surface);
                }
            }
        }
        
        writeln!(out, "{ret}").unwrap();
    }
}
