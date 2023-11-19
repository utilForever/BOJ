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
 
    let (n, m, mut s) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut stack = Vec::new();
    let mut ret = Vec::new();
 
    for i in 1..n {
        while !stack.is_empty() && ret.len() < m {
            if s < n + n * (n - 1) / 2 - i {
                break;
            }
 
            s -= n - i;
            ret.push(stack.pop().unwrap());
        }
 
        ret.push((i, i + 1));
 
        let mut j = 1;
 
        while j < i && stack.len() < m {
            stack.push((j, i + 1));
            j += 1;
        }
    }
 
    while ret.len() < m {
        ret.push(stack.pop().unwrap());
    }
 
    if ret.len() == m && s == n * (n - 1) / 2 {
        for (i, j) in ret {
            writeln!(out, "{i} {j}").unwrap();
        }
    } else {
        writeln!(out, "-1").unwrap();
    }
}
