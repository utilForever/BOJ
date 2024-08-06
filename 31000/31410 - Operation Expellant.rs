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
    let mut pollutions = vec![(0, 0); n];
    let mut pollution_total = 0;
    let mut leak_forward = 0;
    let mut leak_backward = 0;

    for i in 0..n {
        pollutions[i] = (scan.token::<i64>(), scan.token::<i64>());
        pollution_total += pollutions[i].1;
    }

    pollutions.sort();

    for i in 1..n {
        leak_forward += (pollutions[i].0 - pollutions[i - 1].0) * i as i64;
    }

    for i in (0..n - 1).rev() {
        leak_backward += (pollutions[i + 1].0 - pollutions[i].0) * (n - i - 1) as i64;
    }

    let mut ret = i64::MAX;

    // Skip first
    let pollution_remain = pollution_total - pollutions[0].1;
    let leak_remain_forward = leak_forward - (pollutions[n - 1].0 - pollutions[0].0);
    let leak_remain_backward = leak_backward - (pollutions[1].0 - pollutions[0].0) * (n - 1) as i64;

    ret = ret
        .min(pollution_remain + leak_remain_forward)
        .min(pollution_remain + leak_remain_backward);

    // Skip last
    let pollution_remain = pollution_total - pollutions[n - 1].1;
    let leak_remain_forward =
        leak_forward - (pollutions[n - 1].0 - pollutions[n - 2].0) * (n - 1) as i64;
    let leak_remain_backward = leak_backward - (pollutions[n - 1].0 - pollutions[0].0);

    ret = ret
        .min(pollution_remain + leak_remain_forward)
        .min(pollution_remain + leak_remain_backward);

    // Skip middle
    for i in 1..n - 1 {
        let pollution_remain = pollution_total - pollutions[i].1;
        let leak_remain_forward = leak_forward - (pollutions[n - 1].0 - pollutions[i].0);
        let leak_remain_backward = leak_backward - (pollutions[i].0 - pollutions[0].0);

        ret = ret
            .min(pollution_remain + leak_remain_forward)
            .min(pollution_remain + leak_remain_backward);
    }

    writeln!(out, "{ret}").unwrap();
}
