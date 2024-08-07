use io::Write;
use std::{cmp, io, str};

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

// References: http://www.secmem.org/blog/2021/07/18/suffix-array-and-lcp/
fn build_suffix_array(s: &String) -> Vec<usize> {
    let chars = s.as_bytes();
    let n = chars.len();
    let m = cmp::max(256, n) + 1;

    let mut suffix_array = vec![0; n];
    let mut r = vec![0; n * 2];
    let mut nr = vec![0; n * 2];
    let mut cnt = vec![0; m];
    let mut idx = vec![0; n];

    for i in 0..n {
        suffix_array[i] = i;
        r[i] = chars[i] as usize;
    }

    let mut d = 1;
    while d < n {
        for i in 0..m {
            cnt[i] = 0;
        }

        for i in 0..n {
            cnt[r[i + d]] += 1;
        }

        for i in 1..m {
            cnt[i] += cnt[i - 1];
        }

        let mut i = n - 1;
        while !i != 0 {
            cnt[r[i + d]] -= 1;
            idx[cnt[r[i + d]]] = i;
            i -= 1;
        }

        for i in 0..m {
            cnt[i] = 0;
        }

        for i in 0..n {
            cnt[r[i]] += 1;
        }

        for i in 1..m {
            cnt[i] += cnt[i - 1];
        }

        let mut i = n - 1;
        while !i != 0 {
            cnt[r[idx[i]]] -= 1;
            suffix_array[cnt[r[idx[i]]]] = idx[i];
            i -= 1;
        }

        nr[suffix_array[0]] = 1;

        for i in 1..n {
            let a = suffix_array[i - 1];
            let b = suffix_array[i];

            nr[b] = nr[a]
                + if r[a] < r[b] || (r[a] == r[b] && r[a + d] < r[b + d]) {
                    1
                } else {
                    0
                }
        }

        for i in 0..n {
            r[i] = nr[i];
        }

        if r[suffix_array[n - 1]] == n {
            break;
        }

        d <<= 1;
    }

    suffix_array
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let suffix_array = build_suffix_array(&s);
    let mut s = s.chars().collect::<Vec<_>>();
    let mut ret = false;

    for i in 0..s.len() {
        if s[i] == 'a' {
            continue;
        }

        s[i] = (s[i] as u8 - 1) as char;

        let suffix_array_modified = build_suffix_array(&s.iter().collect());

        if suffix_array_modified == suffix_array {
            ret = true;
            break;
        }

        s[i] = (s[i] as u8 + 1) as char;
    }

    writeln!(out, "{}", if ret { "1" } else { "0" }).unwrap();
}
