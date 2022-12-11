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

    let n = scan.token::<usize>();
    let mut snows = vec![0; n];
    let mut ret = 0;

    for i in 0..n {
        snows[i] = scan.token::<i64>();

        if snows[i] > 1440 {
            ret = -1;
        }
    }

    if ret != -1 {
        while !snows.is_empty() {
            snows.sort_by(|a, b| b.cmp(a));
            
            if snows.len() == 1 {
                ret += snows[0];
                break;
            }

            let val = snows[0].min(snows[1]);

            ret += val;
            snows[0] -= val;
            snows[1] -= val;
            
            snows.retain(|&x| x > 0);
        }
    }

    writeln!(out, "{}", if ret > 1440 { -1 } else { ret }).unwrap();
}
