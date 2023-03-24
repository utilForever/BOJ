use io::Write;
use std::{collections::VecDeque, io, str};

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

    let (n, k) = (scan.token::<i64>(), scan.token::<i64>());
    let mut queues = vec![VecDeque::new(); 21];
    let mut ret = 0;

    for i in 0..n {
        let name = scan.token::<String>();

        while !queues[name.len()].is_empty() {
            let val = queues[name.len()].front().unwrap();

            if i - val <= k {
                ret += queues[name.len()].len() as i64;
                break;
            } else {
                queues[name.len()].pop_front();
            }
        }

        queues[name.len()].push_back(i);
    }

    writeln!(out, "{}", ret).unwrap();
}
