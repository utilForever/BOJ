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

    let s = scan.token::<String>();
    let t = scan.token::<String>();

    let mut lcs_count = vec![vec![0; 1001]; 1001];
    let mut i = 0;
    let mut j;

    let s_chars = s.as_bytes();
    let t_chars = t.as_bytes();
    let s_len = s_chars.len();
    let t_len = t_chars.len();

    for s_idx in 0..s_len {
        i += 1;
        j = 0;

        for t_idx in 0..t_len {
            j += 1;

            if s_chars[s_idx] != t_chars[t_idx] {
                if lcs_count[i - 1][j] > lcs_count[i][j - 1] {
                    lcs_count[i][j] = lcs_count[i - 1][j];
                } else {
                    lcs_count[i][j] = lcs_count[i][j - 1];
                }
            } else {
                lcs_count[i][j] = lcs_count[i - 1][j - 1] + 1;
            }
        }
    }

    let mut lcs_str = String::new();
    let (mut i, mut j) = (s_len, t_len);

    while i != 0 && j != 0 {
        let now = lcs_count[i][j];

        if now != lcs_count[i - 1][j] && now != lcs_count[i][j - 1] {
            lcs_str.push(s_chars[i - 1] as char);
        }

        if now == lcs_count[i - 1][j] {
            i -= 1;
        } else if now == lcs_count[i][j - 1] {
            j -= 1;
        } else {
            i -= 1;
            j -= 1;
        }

        if now == 0 {
            break;
        }
    }

    writeln!(out, "{}", lcs_count[s_len][t_len]).unwrap();
    writeln!(out, "{}", lcs_str.chars().rev().collect::<String>()).unwrap();
}
