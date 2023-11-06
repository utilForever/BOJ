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

    let (n, _) = (scan.token::<usize>(), scan.token::<i64>());
    let mut num_blacks = vec![0; n];

    for i in 0..n {
        let s = scan.token::<String>();
        let mut cnt = 0;
        let mut is_black = false;

        for color in s.chars() {
            if color == '1' {
                if !is_black {
                    is_black = true;
                }
            } else {
                if is_black {
                    cnt += 1;
                    is_black = false;
                }
            }
        }

        if is_black {
            cnt += 1;
        }

        num_blacks[i] = cnt;
    }

    let ret_max = *num_blacks.iter().max().unwrap();
    let ret_cnt = num_blacks.iter().filter(|&x| *x == ret_max).count();

    writeln!(out, "{ret_max} {ret_cnt}").unwrap();
}
