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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let s = scan.token::<String>();
    let mut alphabet = vec![Vec::new(); 26];

    for (i, c) in s.chars().enumerate() {
        alphabet[c as usize - 'a' as usize].push(i);
    }

    let mut idx = 0;

    for i in 0..26 {
        if alphabet[i].is_empty() {
            continue;
        }

        let cnt = alphabet[i].len();

        if idx + cnt <= m {
            alphabet[i].clear();
        } else {
            alphabet[i] = alphabet[i].split_off(m - idx);
        }

        idx += cnt;

        if idx >= m {
            break;
        }
    }

    let mut flatten = Vec::with_capacity(n - m);

    for i in 0..26 {
        for j in 0..alphabet[i].len() {
            flatten.push((alphabet[i][j], i));
        }
    }

    flatten.sort();

    let mut ret = String::new();

    for (_, c) in flatten {
        ret.push((c as u8 + 'a' as u8) as char);
    }

    writeln!(out, "{ret}").unwrap();
}
