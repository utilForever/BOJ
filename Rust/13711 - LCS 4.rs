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

fn binary_search(lis: &Vec<i64>, value: i64) -> i64 {
    let mut left = 0;
    let mut right = lis.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let lis_mid_left = if mid > 0 { lis[mid - 1] } else { 0 };

        if (lis_mid_left < value && value < lis[mid]) || lis[mid] == value {
            return mid as i64;
        } else if lis[mid] < value {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}

fn process_lis(sequence: &Vec<i64>, lis: &mut Vec<i64>, bound: usize) {
    lis.push(-1_000_000_007);

    for i in 1..=bound {
        let index = binary_search(&lis, sequence[i]);

        if index == -1 {
            lis.push(sequence[i]);
        } else if sequence[i as usize] < lis[index as usize] {
            lis[index as usize] = lis[index as usize].min(sequence[i]);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
    }

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    let mut temp = vec![0; n];
    let mut c = vec![0; n];

    for i in 0..n {
        temp[b[i] as usize - 1] = i as i64;
    }

    for i in 0..n {
        c[i] = temp[a[i] as usize - 1];
    }

    let mut lis = Vec::new();
    c.insert(0, 0);

    process_lis(&c, &mut lis, n);

    writeln!(out, "{}", lis.len() - 1).unwrap();
}
