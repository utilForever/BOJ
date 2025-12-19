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

fn build_palindrome(prefix: i64, len_prefix: usize, len_total: usize) -> i64 {
    let prefix_str = format!("{:0width$}", prefix, width = len_prefix);
    let mut s = prefix_str.clone();

    if len_total % 2 == 0 {
        let rev = prefix_str.chars().rev().collect::<String>();
        s.push_str(&rev);
    } else {
        let left = &prefix_str[..len_prefix - 1];
        let rev = left.chars().rev().collect::<String>();
        s.push_str(&rev);
    }

    s.parse::<i64>().unwrap()
}

fn calculate_palindrome_max(n: i64) -> i64 {
    if n < 10 {
        return n;
    }

    let s = n.to_string();
    let len_total = s.len();
    let len_prefix = (len_total + 1) / 2;
    let prefix1 = s[..len_prefix].parse::<i64>().unwrap();

    let candidate = build_palindrome(prefix1, len_prefix, len_total);

    if candidate <= n {
        return candidate;
    }

    let prefix2 = prefix1 - 1;
    let min_prefix = 10i64.pow((len_prefix - 1) as u32);

    if prefix2 < min_prefix {
        return 10i64.pow((len_total - 1) as u32) - 1;
    }

    build_palindrome(prefix2, len_prefix, len_total)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = scan.token::<i64>();
    let mut ret = Vec::new();

    while n > 0 {
        let p = calculate_palindrome_max(n);

        ret.push(p);
        n -= p;
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
