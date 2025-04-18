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

fn select(
    arr: &mut Vec<i64>,
    p: usize,
    r: usize,
    q: usize,
    k: usize,
    cnt: &mut usize,
    ret: &mut Option<(i64, i64)>,
) -> i64 {
    if p == r {
        return arr[p];
    }

    let t = partition(arr, p, r, k, cnt, ret);
    let s = t - p + 1;

    if q < s {
        select(arr, p, t - 1, q, k, cnt, ret)
    } else if q == s {
        arr[t]
    } else {
        select(arr, t + 1, r, q - s, k, cnt, ret)
    }
}

fn partition(
    arr: &mut Vec<i64>,
    p: usize,
    r: usize,
    k: usize,
    cnt: &mut usize,
    ret: &mut Option<(i64, i64)>,
) -> usize {
    let x = arr[r];
    let mut i = p - 1;

    for j in p..r {
        if arr[j] <= x {
            i += 1;
            arr.swap(i, j);
            *cnt += 1;

            if *cnt == k {
                *ret = Some((arr[i], arr[j]));
            }
        }
    }

    if i + 1 != r {
        arr.swap(i + 1, r);
        *cnt += 1;

        if *cnt == k {
            *ret = Some((arr[i + 1], arr[r]));
        }
    }

    i + 1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let mut cnt = 0;
    let mut ret = None;

    select(&mut nums, 1, n, q, k, &mut cnt, &mut ret);

    if let Some(ret) = ret {
        writeln!(out, "{} {}", ret.0, ret.1).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
