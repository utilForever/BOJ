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
    let mut immunities = vec![0; n];

    for i in 0..n {
        immunities[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + immunities[i];
    }

    let mut exits_available = vec![0; n + 1];

    for i in 1..=n {
        exits_available[i] = immunities[i - 1] - prefix_sum[i - 1];
    }

    let mut exits_max = vec![0; n + 1];
    exits_max[1] = exits_available[1];

    for i in 2..=n {
        exits_max[i] = exits_max[i - 1].max(exits_available[i]);
    }

    let mut immunity_curr = 0;
    let mut ret = 0;

    while immunity_curr < prefix_sum[n] {
        let idx = match prefix_sum.binary_search(&immunity_curr) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };

        if idx == n {
            break;
        }

        let immunity_next = prefix_sum[idx + 1];
        let immunity_increase = exits_max[idx + 1];
        let diff = immunity_next - immunity_curr;

        let r = (immunity_increase + diff - 1) / immunity_increase;
        immunity_curr += r * immunity_increase;
        ret += r;
    }

    writeln!(out, "{ret}").unwrap();
}
