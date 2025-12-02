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

fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }

    x
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(n);

    for _ in 0..n {
        let (d, a) = (scan.token::<char>(), scan.token::<usize>());

        if d == 'L' {
            left.push(a);
        } else {
            right.push(a);
        }
    }

    let mut cnt_left = vec![0; n + 1];
    let mut prefix_sum_left = vec![0; n + 1];

    for i in 0..left.len() {
        cnt_left[left[i]] += 1;
    }

    for i in 1..=n {
        prefix_sum_left[i] = prefix_sum_left[i - 1] + cnt_left[i];
    }

    let mut cnt_right = vec![0; n + 1];
    let mut prefix_sum_right = vec![0; n + 1];

    for i in 0..right.len() {
        cnt_right[right[i]] += 1;
    }

    for i in 1..=n {
        prefix_sum_right[i] = prefix_sum_right[i - 1] + cnt_right[i];
    }

    let mut lower_bound = vec![0; n + 1];

    for i in 1..=n {
        let val = (left.len() as i64 - (n - i) as i64 + prefix_sum_right[n - i]).max(0);
        lower_bound[i] = (prefix_sum_left[i] as i64).max(val);
    }

    let mut lower_bound_prefix_max = vec![0; n + 1];

    for i in 1..=n {
        lower_bound_prefix_max[i] = lower_bound_prefix_max[i - 1].max(lower_bound[i]);
    }

    for i in 0..=n {
        if lower_bound_prefix_max[i] > i as i64 {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    if lower_bound_prefix_max[n] > left.len() as i64 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut deadline = vec![0; n + 1];

    for i in 1..=n {
        deadline[i] = lower_bound_prefix_max[i] - lower_bound_prefix_max[i - 1];
    }

    let mut parent = (0..=n).collect::<Vec<_>>();
    let mut used = vec![false; n + 1];

    for i in 1..=n {
        let cnt = deadline[i] as usize;

        for _ in 0..cnt {
            let s = find(&mut parent, i);

            if s == 0 {
                writeln!(out, "-1").unwrap();
                return;
            }

            used[s] = true;

            let root = find(&mut parent, s - 1);
            parent[s] = root;
        }
    }

    let mut sum = 0;

    for i in 1..=n {
        if used[i] {
            sum += i;
        }
    }

    writeln!(out, "{}", 2 * sum + right.len() * (n + 1) - n * (n + 1) / 2).unwrap();
}
