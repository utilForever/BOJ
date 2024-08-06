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

    let _ = scan.token::<i64>();
    let s = scan.token::<String>();
    let mut cnt = [(0, 'S'), (0, 'B'), (0, 'A')];

    for ch in s.chars() {
        match ch {
            'S' => cnt[0].0 += 1,
            'B' => cnt[1].0 += 1,
            'A' => cnt[2].0 += 1,
            _ => unreachable!(),
        }
    }

    cnt.sort_by(|a, b| b.0.cmp(&a.0));

    if cnt[0].0 == cnt[1].0 {
        if cnt[1].0 == cnt[2].0 {
            writeln!(out, "SCU").unwrap();
        } else {
            if cnt[0].1 == 'B' {
                if cnt[1].1 == 'S' {
                    writeln!(out, "BS").unwrap();
                } else {
                    writeln!(out, "BA").unwrap();
                }
            } else if cnt[0].1 == 'S' {
                if cnt[1].1 == 'B' {
                    writeln!(out, "BS").unwrap();
                } else {
                    writeln!(out, "SA").unwrap();
                }
            } else {
                if cnt[1].1 == 'B' {
                    writeln!(out, "BA").unwrap();
                } else {
                    writeln!(out, "SA").unwrap();
                }
            }
        }
    } else {
        writeln!(out, "{}", cnt[0].1).unwrap();
    }
}
