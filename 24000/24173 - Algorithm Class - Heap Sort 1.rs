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

fn heap_sort(
    arr: &mut Vec<i64>,
    n: usize,
    k: usize,
    cnt: &mut usize,
    ret: &mut Option<(i64, i64)>,
) {
    build_min_heap(arr, n, k, cnt, ret);

    for i in (2..=n).rev() {
        arr.swap(1, i);
        *cnt += 1;

        if *cnt == k {
            *ret = Some((arr[1], arr[i]));
        }

        heapify(arr, 1, i - 1, k, cnt, ret);
    }
}

fn build_min_heap(
    arr: &mut Vec<i64>,
    n: usize,
    k: usize,
    cnt: &mut usize,
    ret: &mut Option<(i64, i64)>,
) {
    for i in (1..=n / 2).rev() {
        heapify(arr, i, n, k, cnt, ret);
    }
}

fn heapify(
    arr: &mut Vec<i64>,
    m: usize,
    n: usize,
    k: usize,
    cnt: &mut usize,
    ret: &mut Option<(i64, i64)>,
) {
    let left = 2 * m;
    let right = 2 * m + 1;

    let smaller = if right <= n {
        if arr[left] < arr[right] {
            left
        } else {
            right
        }
    } else if left <= n {
        left
    } else {
        return;
    };

    if arr[smaller] < arr[m] {
        arr.swap(m, smaller);
        *cnt += 1;

        if *cnt == k {
            *ret = Some((arr[m], arr[smaller]));
        }

        heapify(arr, smaller, n, k, cnt, ret);
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

    heap_sort(&mut nums, n, k, &mut cnt, &mut ret);

    if let Some(ret) = ret {
        writeln!(out, "{} {}", ret.0.min(ret.1), ret.0.max(ret.1)).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
