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

    let t = scan.token::<i64>();
    let nums = [
        111, 117, 119, 171, 231, 237, 297, 319, 371, 411, 413, 417, 437, 471, 473, 531, 537, 597,
        611, 671, 679, 711, 713, 717, 731, 737, 831, 837, 897, 973, 979, 1131, 1137, 1311, 1313,
        1317, 1379, 1797, 1971, 3113, 3131, 3173, 3179, 4197, 4311, 4313, 4317, 4797, 6137, 6179,
        7197, 7971, 31373,
    ];

    for _ in 0..t {
        let num = scan.token::<i64>();
        let pos = nums.partition_point(|&x| x <= num);

        writeln!(out, "{}", if pos == 0 { -1 } else { nums[pos - 1] }).unwrap();
    }
}
