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

fn merge_sort(a: &mut Vec<i64>, b: &Vec<i64>, p: usize, r: usize, idx: &mut usize, ret: &mut bool) {
    if p < r {
        let q = (p + r) / 2;

        merge_sort(a, b, p, q, idx, ret);
        merge_sort(a, b, q + 1, r, idx, ret);
        merge(a, b, p, q, r, idx, ret);
    }
}

fn merge(
    a: &mut Vec<i64>,
    b: &Vec<i64>,
    p: usize,
    q: usize,
    r: usize,
    idx: &mut usize,
    ret: &mut bool,
) {
    let mut tmp = vec![0];
    let mut i = p;
    let mut j = q + 1;
    let mut t = 1;

    while i <= q && j <= r {
        if a[i] <= a[j] {
            tmp.push(a[i]);
            t += 1;
            i += 1;
        } else {
            tmp.push(a[j]);
            t += 1;
            j += 1;
        }
    }

    while i <= q {
        tmp.push(a[i]);
        t += 1;
        i += 1;
    }

    while j <= r {
        tmp.push(a[j]);
        t += 1;
        j += 1;
    }

    i = p;
    t = 1;

    while i <= r {
        a[i] = tmp[t];

        if *ret == false {
            let mut check = true;

            for pos in *idx..a.len() {
                if a[pos] != b[pos] {
                    check = false;
                    break;
                }

                *idx += 1;
            }

            if check {
                *ret = true;
            }
        }

        i += 1;
        t += 1;
    }
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

    let mut idx = 1;
    let mut ret = false;

    merge_sort(&mut a, &b, 1, n, &mut idx, &mut ret);

    writeln!(out, "{}", if ret { 1 } else { 0 }).unwrap();
}
