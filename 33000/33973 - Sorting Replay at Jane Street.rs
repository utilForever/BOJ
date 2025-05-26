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

const MOD: i64 = 998_244_353;

fn calculate(
    nums: &Vec<Vec<i64>>,
    queries: &Vec<(i64, usize)>,
    idxes: &Vec<usize>,
    factorial: &Vec<i64>,
    depth: i64,
) -> i64 {
    if nums.len() <= 1 || depth < 0 {
        return 1;
    }

    let mut groups: HashMap<i64, Vec<usize>> = HashMap::new();

    for &idx in idxes.iter() {
        groups
            .entry(nums[idx][queries[depth as usize].1])
            .or_default()
            .push(idx);
    }

    let mut ret = 1;

    if queries[depth as usize].0 == 1 {
        for (_, idx) in groups {
            ret = (ret * factorial[idx.len()]) % MOD;
        }
    } else {
        for (_, idx) in groups {
            ret = ret * calculate(nums, queries, &idx, factorial, depth - 1) % MOD;
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            nums[i][j] = scan.token::<i64>();
        }
    }

    let mut queries = vec![(0, 0); q];

    for i in 0..q {
        queries[i] = (scan.token::<i64>(), scan.token::<usize>() - 1);
    }

    let idxes = (0..n).collect::<Vec<_>>();
    let mut factorial = vec![1; n + 1];

    for i in 1..=n {
        factorial[i] = (factorial[i - 1] * i as i64) % MOD;
    }

    writeln!(
        out,
        "{}",
        calculate(&nums, &queries, &idxes, &factorial, q as i64 - 1)
    )
    .unwrap();
}
