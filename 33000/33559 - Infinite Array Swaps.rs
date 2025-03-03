use io::Write;
use std::{collections::HashMap, io, str};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
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

    let mut freq_a = HashMap::new();
    let mut freq_b = HashMap::new();

    for &val in a.iter() {
        *freq_a.entry(val).or_insert(0) += 1;
    }

    for &val in b.iter() {
        *freq_b.entry(val).or_insert(0) += 1;
    }

    let mut keys_union = freq_a.keys().cloned().collect::<Vec<_>>();

    for &val in freq_b.keys() {
        if !freq_a.contains_key(&val) {
            keys_union.push(val);
        }
    }

    let mut ret = 0;
    let mut ret_common = Vec::new();
    let mut ret_a_remain = Vec::new();
    let mut ret_b_remain = Vec::new();

    for &val in keys_union.iter() {
        let count_a = *freq_a.get(&val).unwrap_or(&0);
        let count_b = *freq_b.get(&val).unwrap_or(&0);
        let common = count_a.min(count_b);

        ret += common;

        for _ in 0..common {
            ret_common.push(val);
        }

        for _ in 0..(count_a - common) {
            ret_a_remain.push(val);
        }

        for _ in 0..(count_b - common) {
            ret_b_remain.push(val);
        }
    }

    let mut ret_a = Vec::with_capacity(n);
    let mut ret_b = Vec::with_capacity(n);

    ret_a.extend(ret_common.iter().cloned());
    ret_a.extend(ret_a_remain.iter().cloned());
    ret_b.extend(ret_common.iter().cloned());
    ret_b.extend(ret_b_remain.iter().cloned());

    writeln!(out, "{ret}").unwrap();

    for i in 0..n {
        write!(out, "{} ", ret_a[i]).unwrap();
    }

    writeln!(out).unwrap();

    for i in 0..n {
        write!(out, "{} ", ret_b[i]).unwrap();
    }

    writeln!(out).unwrap();
}
