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

fn merge_sort(
    arr: &mut Vec<i64>,
    left: usize,
    right: usize,
    k: usize,
    save: &mut i64,
    cnt_save: &mut usize,
) {
    if left >= right {
        return;
    }

    let mid = (left + right) / 2;

    merge_sort(arr, left, mid, k, save, cnt_save);
    merge_sort(arr, mid + 1, right, k, save, cnt_save);
    merge(arr, left, mid, right, k, save, cnt_save);
}

fn merge(
    arr: &mut Vec<i64>,
    left: usize,
    mid: usize,
    right: usize,
    k: usize,
    save: &mut i64,
    cnt_save: &mut usize,
) {
    let mut temp = vec![0; right - left + 1];
    let mut i = left;
    let mut j = mid + 1;
    let mut t = 0;

    while i <= mid && j <= right {
        if arr[i] <= arr[j] {
            temp[t as usize] = arr[i];
            t += 1;
            i += 1;
        } else {
            temp[t as usize] = arr[j];
            t += 1;
            j += 1;
        }
    }

    while i <= mid {
        temp[t as usize] = arr[i];
        t += 1;
        i += 1;
    }

    while j <= right {
        temp[t as usize] = arr[j];
        t += 1;
        j += 1;
    }

    i = left;
    t = 0;

    while i <= right {
        arr[i] = temp[t as usize];
        *cnt_save += 1;

        if *cnt_save == k {
            *save = arr[i];
        }

        t += 1;
        i += 1;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n];
    let mut save = 0;
    let mut cnt_save = 0;

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    merge_sort(&mut nums, 0, n - 1, k, &mut save, &mut cnt_save);

    writeln!(out, "{}", if cnt_save < k { -1 } else { save }).unwrap();
}
