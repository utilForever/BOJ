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

    let n = scan.token::<usize>();
    let mut blocks = vec![0; n];

    for i in 0..n {
        blocks[i] = scan.token::<i64>();
    }

    let sum_block = blocks.iter().sum::<i64>();
    let mut heights = vec![0; n];
    let mut prev = 0;

    for (idx, &block) in blocks.iter().enumerate() {
        if (idx + 1) % 2 == 0 {
            let mut val = prev + 1;

            if val & 1 != block & 1 {
                val += 1;
            }

            heights[idx] = val;
        } else {
            let mut val = i64::MAX;

            if block > prev {
                val = block;
            }

            let mut candidate = prev + 1;

            if (candidate - block) & 1 == 0 {
                candidate += 1;
            }

            val = val.min(candidate);
            heights[idx] = val;
        }

        prev = heights[idx];
    }

    let mut sum_height = heights.iter().sum::<i64>();

    if sum_height > sum_block {
        writeln!(out, "NO").unwrap();
        return;
    }

    let mut remain = sum_block - sum_height;

    if remain > 0 {
        let delta = heights[n - 1] - blocks[n - 1];

        if n % 2 == 1 && (delta & 1) == 0 && remain >= 2 {
            heights[n - 1] += 1;
            remain -= 1;
        }

        let remove = remain & 1;
        let add_even = remain - remove;

        heights[n - 1] += add_even;
        sum_height += add_even
            + if n % 2 == 1 && (delta & 1) == 0 && remain >= 1 {
                1
            } else {
                0
            };

        if sum_height != sum_block - remove {
            writeln!(out, "NO").unwrap();
            return;
        }
    }

    writeln!(out, "YES").unwrap();

    for i in 0..n {
        write!(out, "{} ", heights[i] - blocks[i]).unwrap();
    }

    writeln!(out).unwrap();
}
