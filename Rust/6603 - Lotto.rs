use io::Write;
use std::{
    io::{self, BufWriter, StdoutLock},
    str,
};

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

fn process_dfs(out: &mut BufWriter<StdoutLock>, nums: &Vec<i64>, lotto: &mut Vec<i64>, idx: usize) {
    if lotto.len() == 6 {
        for num in lotto.iter() {
            write!(out, "{} ", num).unwrap();
        }

        writeln!(out).unwrap();
        return;
    }

    for i in idx..nums.len() {
        lotto.push(nums[i]);
        process_dfs(out, nums, lotto, i + 1);
        lotto.pop();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let k = scan.token::<usize>();
        if k == 0 {
            break;
        }

        let mut nums = vec![0; k];
        let mut lotto = Vec::new();

        for i in 0..k {
            nums[i] = scan.token::<i64>();
        }

        nums.sort();

        process_dfs(&mut out, &nums, &mut lotto, 0);

        lotto.clear();
        writeln!(out).unwrap();
    }
}
