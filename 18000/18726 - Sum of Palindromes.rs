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

fn build_palindrome(prefix: &Vec<u8>, len_total: usize) -> Vec<u8> {
    let len_prefix = prefix.len();
    let mut ret = Vec::with_capacity(len_total);

    ret.extend_from_slice(prefix);

    if len_total % 2 == 0 {
        for &c in prefix.iter().rev() {
            ret.push(c);
        }
    } else {
        for &c in prefix[..len_prefix - 1].iter().rev() {
            ret.push(c);
        }
    }

    ret
}

fn compare_num(a: &Vec<u8>, b: &Vec<u8>) -> std::cmp::Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }

    a.cmp(b)
}

fn decrease_prefix(mut prefix: Vec<u8>) -> Option<Vec<u8>> {
    let mut idx = prefix.len();

    while idx > 0 {
        idx -= 1;

        if prefix[idx] > b'0' {
            prefix[idx] -= 1;
            break;
        } else {
            prefix[idx] = b'9';
        }
    }

    if prefix[0] == b'0' {
        None
    } else {
        Some(prefix)
    }
}

fn calculate_palindrome_max(n: &Vec<u8>) -> Vec<u8> {
    if n.len() == 1 {
        return n.to_vec();
    }

    let len_total = n.len();
    let len_prefix = (len_total + 1) / 2;
    let prefix1 = n[..len_prefix].to_vec();

    let candidate = build_palindrome(&prefix1, len_total);

    if compare_num(&candidate, n) != std::cmp::Ordering::Greater {
        return candidate;
    }

    match decrease_prefix(prefix1) {
        Some(prefix2) => build_palindrome(&prefix2, len_total),
        None => vec![b'9'; len_total - 1],
    }
}

fn subtract(a: &mut Vec<u8>, b: &Vec<u8>) {
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;
    let mut borrow = 0;

    while i >= 0 {
        let digit_a = (a[i as usize] - b'0') as i64;
        let digit_b = if j >= 0 {
            (b[j as usize] - b'0') as i64
        } else {
            0
        };

        let mut val = digit_a - digit_b - borrow;

        if val < 0 {
            val += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        a[i as usize] = (val as u8) + b'0';
        i -= 1;
        j -= 1;
    }

    let mut pos = 0;

    while pos + 1 < a.len() && a[pos] == b'0' {
        pos += 1;
    }

    if pos > 0 {
        *a = a[pos..].to_vec();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let z = scan.token::<i64>();

    for _ in 0..z {
        let mut n = scan.token::<String>().bytes().collect::<Vec<_>>();
        let mut ret = Vec::new();

        while !(n.len() == 1 && n[0] == b'0') {
            let p = calculate_palindrome_max(&n);
            subtract(&mut n, &p);
            ret.push(p);
        }

        writeln!(out, "{}", ret.len()).unwrap();

        for val in ret {
            writeln!(out, "{}", String::from_utf8(val).unwrap()).unwrap();
        }
    }
}
