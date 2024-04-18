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

fn process_dfs(
    nums: &Vec<usize>,
    visited: &mut Vec<bool>,
    checked: &mut Vec<bool>,
    ret: &mut usize,
    pos: usize,
) {
    visited[pos] = true;

    if !visited[nums[pos]] {
        process_dfs(nums, visited, checked, ret, nums[pos]);
    } else if !checked[nums[pos]] {
        let mut idx = nums[pos];

        while idx != pos {
            idx = nums[idx];
            *ret += 1;
        }

        *ret += 1;
    }

    checked[pos] = true;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut nums = vec![0; n + 1];
        let mut visited = vec![false; n + 1];
        let mut checked = vec![false; n + 1];
        let mut ret = 0;

        for i in 1..=n {
            nums[i] = scan.token::<usize>();
        }

        for i in 1..=n {
            if visited[i] {
                continue;
            }

            process_dfs(&nums, &mut visited, &mut checked, &mut ret, i);
        }

        writeln!(out, "{}", n - ret).unwrap();
    }
}
