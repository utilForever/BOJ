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
    let r = scan.token::<String>();

    let mut lcs_count = vec![vec![vec![0; 101]; 101]; 101];
    let mut i = 0;
    let mut j;
    let mut k;

    for s in s.chars() {
        i += 1;
        j = 0;

        for t in t.chars() {
            j += 1;
            k = 0;

            for r in r.chars() {
                k += 1;

                if s != t || t != r || s != r {
                    lcs_count[i][j][k] = *vec![
                        lcs_count[i - 1][j][k],
                        lcs_count[i][j - 1][k],
                        lcs_count[i][j][k - 1],
                    ]
                    .iter()
                    .max()
                    .unwrap();
                } else {
                    lcs_count[i][j][k] = lcs_count[i - 1][j - 1][k - 1] + 1;
                }
            }
        }
    }

    writeln!(out, 
        "{}",
        lcs_count[s.chars().count()][t.chars().count()][r.chars().count()]
    ).unwrap();
}
