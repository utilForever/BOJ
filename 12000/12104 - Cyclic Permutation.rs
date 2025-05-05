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

fn calculate_fail_function(pattern: &Vec<char>) -> Vec<i64> {
    let n = pattern.len();
    let mut cmp = 0;
    let mut fail = vec![0; n];

    for i in 1..n {
        while cmp > 0 && pattern[cmp] != pattern[i] {
            cmp = fail[cmp - 1] as usize;
        }

        if pattern[cmp] == pattern[i] {
            cmp += 1;
            fail[i] = cmp as i64;
        }
    }

    fail
}

fn process_kmp(s: &Vec<char>, pattern: &Vec<char>) -> i64 {
    let n = s.len();
    let fail = calculate_fail_function(pattern);
    let mut idx = 0;
    let mut cnt = 0;

    for i in 0..n - 1 {
        while idx > 0 && s[i] != pattern[idx] {
            idx = fail[idx - 1] as usize;
        }

        if s[i] == pattern[idx] {
            if idx == pattern.len() - 1 {
                idx = fail[idx] as usize;
                cnt += 1;
            } else {
                idx += 1;
            }
        }
    }

    cnt
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut a = scan.token::<String>().chars().collect::<Vec<_>>();
    let b = scan.token::<String>().chars().collect::<Vec<_>>();
    let n = a.len();

    for i in 0..n {
        a.push(a[i]);
    }

    let ret = process_kmp(&a, &b);

    writeln!(out, "{ret}").unwrap();
}
