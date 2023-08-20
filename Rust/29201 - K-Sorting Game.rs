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

#[derive(Clone, Debug)]
struct Node {
    val: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self { val }
    }
}

struct FenwickTree {
    data: Vec<Node>,
}

impl FenwickTree {
    pub fn new() -> Self {
        Self {
            data: vec![Node::new(0); 1 << 20],
        }
    }

    pub fn update(&mut self, idx: usize, num: i64) {
        let mut idx = idx as i64;

        while idx <= num {
            self.data[idx as usize].val += 1;
            idx += idx & -idx;
        }
    }

    pub fn query(&mut self, idx: usize) -> Node {
        let mut idx = idx as i64;
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx as usize].val;
            idx -= idx & -idx;
        }

        Node::new(ret)
    }
}

fn inverse(nums: &Vec<i64>, n: usize) -> i64 {
    let mut tree = FenwickTree::new();
    let mut ret = 0;

    for i in 0..nums.len() {
        ret += i as i64 - tree.query(nums[i] as usize).val;
        tree.update(nums[i] as usize, n as i64);
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>() - 1);
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in (0..=k - 1).rev() {
        left.push(nums[i as usize]);
    }

    for i in k + 1..n as i64 {
        right.push(nums[i as usize]);
    }

    if !left.contains(&1) {
        let tmp = left;
        left = right;
        right = tmp;
    }

    left.insert(0, nums[k as usize]);

    let inverse_left = inverse(&left, n);
    let inverse_right = inverse(&right, n);

    if nums[k as usize] == 1 {
        writeln!(
            out,
            "{}",
            if (inverse_left + inverse_right) % 2 == 1 {
                "Minchan"
            } else {
                "Junee"
            }
        )
        .unwrap();
    } else if (inverse_left + inverse_right) % 2 == 1 {
        writeln!(out, "Minchan").unwrap();
    } else {
        let mut min = i64::MAX;

        if !right.is_empty() {
            min = *right.iter().min().unwrap();
        }

        writeln!(
            out,
            "{}",
            if nums[k as usize] > min && min % 2 != nums[k as usize] % 2 {
                "Minchan"
            } else {
                "Junee"
            }
        )
        .unwrap();
    }
}
