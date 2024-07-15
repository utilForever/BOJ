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

// References: http://www.secmem.org/blog/2021/07/18/suffix-array-and-lcp/
fn build_lcp_array(s: &String, suffix_array: &Vec<usize>) -> Vec<usize> {
    let chars = s.as_bytes();
    let s_len = chars.len();
    let mut lcp_array = vec![0; s_len];
    let mut inverse_suffix_array = vec![0; s_len];

    for i in 0..s_len {
        inverse_suffix_array[suffix_array[i]] = i;
    }

    let mut k = 0;

    for i in 0..s_len {
        if inverse_suffix_array[i] != 0 {
            let j = suffix_array[inverse_suffix_array[i] - 1];

            while i + k < s_len && j + k < s_len && chars[i + k] == chars[j + k] {
                k += 1;
            }

            lcp_array[inverse_suffix_array[i]] = if k > 0 { k } else { 0 };

            if k > 0 {
                k -= 1;
            }
        }
    }

    lcp_array
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        let suffix_array = build_suffix_array(&s);
        let lcp_array = build_lcp_array(&s, &suffix_array);
        let mut ret = lcp_array.iter().sum::<usize>();

        for i in 0..s.len() - 1 {
            ret -= lcp_array[i].min(lcp_array[i + 1]);
        }

        writeln!(out, "{ret}").unwrap();
    }
}
