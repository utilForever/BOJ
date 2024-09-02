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
 
    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
 
    let n = scan.token::<usize>();
    let mut nums = vec![0; n + 1];
 
    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }
 
    let mut prefix_sum = vec![(0, 0); n + 1];
 
    for i in 1..=n {
        prefix_sum[i] = (prefix_sum[i - 1].0 + nums[i], i);
    }
 
    prefix_sum.sort();
 
    let mut ret_sum = prefix_sum[1].0 - prefix_sum[0].0;
    let mut ret_left = prefix_sum[0].1;
    let mut ret_right = prefix_sum[1].1;
 
    for i in 1..n {
        if prefix_sum[i + 1].0 - prefix_sum[i].0 < ret_sum {
            ret_sum = prefix_sum[i + 1].0 - prefix_sum[i].0;
            ret_left = prefix_sum[i].1;
            ret_right = prefix_sum[i + 1].1;
        }
    }
 
    if ret_left >= ret_right {
        ret_sum *= -1;
    }
 
    if ret_left > ret_right {
        std::mem::swap(&mut ret_left, &mut ret_right);
    }
 
    ret_left += 1;
 
    writeln!(out, "{ret_sum}").unwrap();
    writeln!(out, "{ret_left} {ret_right}").unwrap();
}
