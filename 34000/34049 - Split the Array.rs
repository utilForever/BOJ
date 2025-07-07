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
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
    }

    let mut next = vec![n + 1; n + 1];
    let mut stack = Vec::new();

    for i in (0..=n).rev() {
        while stack
            .last()
            .map_or(false, |&j| prefix_sum[j] >= prefix_sum[i])
        {
            stack.pop();
        }

        next[i] = *stack.last().unwrap_or(&(n + 1));
        stack.push(i);
    }

    let mut dp = vec![-1; n + 1];
    let mut good = vec![n + 1; n + 2];

    dp[n] = 0;
    good[n] = n;

    for i in (0..n).rev() {
        let j = good[i + 1];

        if j < next[i] && dp[j] != -1 {
            dp[i] = dp[j] + 1;
        }

        good[i] = if dp[i] != -1 { i } else { good[i + 1] };
    }

    writeln!(out, "{}", dp[0]).unwrap();
}
