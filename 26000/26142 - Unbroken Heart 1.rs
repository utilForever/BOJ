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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut dragons = vec![(0, 0); n];
    let mut shoot = vec![false; n];

    for i in 0..n {
        dragons[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = 0;

    // Sort by increasing height for greedy approach
    dragons.sort_by(|a, b| b.0.cmp(&a.0));

    for i in 0..n {
        let mut idx_cur = i as i64;
        let mut idx_ret = 0;
        let mut height_max = -1;
        let mut height_sum = ret;

        for j in 0..n {
            if shoot[j] {
                // If dragon is already shot, add its height to the sum
                height_sum += dragons[j].0;
                idx_cur -= 1;
            } else if height_max < height_sum + idx_cur * dragons[j].0 + dragons[j].1 {
                // If dragon is not shot, check if it is the best candidate
                height_max = height_sum + idx_cur * dragons[j].0 + dragons[j].1;
                idx_ret = j as i64;
            }
        }

        ret = height_max;
        shoot[idx_ret as usize] = true;

        writeln!(out, "{ret}").unwrap();
    }
}
