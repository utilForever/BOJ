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

fn merge_sort(
    arr: &mut Vec<i64>,
    p: usize,
    r: usize,
    k: usize,
    cnt: &mut usize,
    ret: &mut Option<i64>,
) {
    if p < r {
        let q = (p + r) / 2;

        merge_sort(arr, p, q, k, cnt, ret);
        merge_sort(arr, q + 1, r, k, cnt, ret);
        merge(arr, p, q, r, k, cnt, ret);
    }
}

fn merge(
    arr: &mut Vec<i64>,
    p: usize,
    q: usize,
    r: usize,
    k: usize,
    cnt: &mut usize,
    ret: &mut Option<i64>,
) {
    let mut tmp = vec![0];
    let mut i = p;
    let mut j = q + 1;
    let mut t = 1;

    while i <= q && j <= r {
        if arr[i] <= arr[j] {
            tmp.push(arr[i]);
            t += 1;
            i += 1;
        } else {
            tmp.push(arr[j]);
            t += 1;
            j += 1;
        }
    }

    while i <= q {
        tmp.push(arr[i]);
        t += 1;
        i += 1;
    }

    while j <= r {
        tmp.push(arr[j]);
        t += 1;
        j += 1;
    }

    i = p;
    t = 1;

    while i <= r {
        arr[i] = tmp[t];
        *cnt += 1;

        if *cnt == k {
            *ret = Some(arr[i]);
        }

        i += 1;
        t += 1;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let mut cnt = 0;
    let mut ret = None;

    merge_sort(&mut nums, 1, n, k, &mut cnt, &mut ret);

    if let Some(ret) = ret {
        writeln!(out, "{ret}").unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
