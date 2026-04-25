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

const BIT: usize = 20;
const MASK: usize = 1 << BIT;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<usize>();
    }

    let mut prefix_xor = vec![0; n + 1];
    let mut idx_first = vec![usize::MAX; MASK];

    for i in 1..=n {
        prefix_xor[i] = prefix_xor[i - 1] ^ nums[i - 1];
        
        if idx_first[prefix_xor[i]] == usize::MAX {
            idx_first[prefix_xor[i]] = i;
        }
    }

    let mut dp = idx_first;

    for b in 0..BIT {
        let mut base = 0;

        while base < MASK {
            for offset in 0..(1 << b) {
                dp[base + offset] = dp[base + offset].min(dp[base + offset + (1 << b)]);
            }

            base += (1 << b) * 2;
        }
    }

    for i in 1..=n {
        let mut ret = 0;

        for b in (0..BIT).rev() {
            if (((MASK - 1) ^ prefix_xor[i]) & (1 << b)) != 0 {
                let candidate = ret | (1 << b);

                if dp[candidate] <= i {
                    ret = candidate;
                }
            }
        }

        write!(out, "{} ", prefix_xor[i] + 2 * ret).unwrap();
    }

    writeln!(out).unwrap();
}
