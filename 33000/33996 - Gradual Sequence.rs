use io::Write;
use std::{collections::HashMap, io, str};

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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    if n < 3 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut vals = vec![HashMap::new(); n];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..i {
            let diff = nums[i] - nums[j];
            let mut cnt = 0;
            let mut len = 0;

            for k in -1..=1 {
                if let Some(&(cnt_prev, len_prev)) = vals[j].get(&(diff + k)) {
                    cnt = (cnt + cnt_prev) % MOD;
                    len = (len + len_prev + cnt_prev) % MOD;
                }
            }

            ret = (ret + len) % MOD;

            let entry = vals[i].entry(diff).or_insert((0, 0));
            entry.0 = (entry.0 + cnt + 1) % MOD;
            entry.1 = (entry.1 + len + 2) % MOD;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
