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

fn quick_sort(a: &mut Vec<i64>, b: &Vec<i64>, p: usize, r: usize, ret: &mut bool) {
    if p < r {
        let q = partition(a, b, p, r, ret);
        quick_sort(a, b, p, q - 1, ret);
        quick_sort(a, b, q + 1, r, ret);
    }
}

fn partition(a: &mut Vec<i64>, b: &Vec<i64>, p: usize, r: usize, ret: &mut bool) -> usize {
    let x = a[r];
    let mut i = p - 1;

    for j in p..r {
        if a[j] <= x {
            i += 1;

            if a[i] != a[j] {
                a.swap(i, j);

                if a == b {
                    *ret = true;
                }
            }
        }
    }

    if i + 1 != r {
        if a[i + 1] != a[r] {
            a.swap(i + 1, r);

            if a == b {
                *ret = true;
            }
        }
    }

    i + 1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];

    for i in 1..=n {
        a[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        b[i] = scan.token::<i64>();
    }

    if a == b {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut ret = false;
    quick_sort(&mut a, &b, 1, n, &mut ret);

    writeln!(out, "{}", if ret { 1 } else { 0 }).unwrap();
}
