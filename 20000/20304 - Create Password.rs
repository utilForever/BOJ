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

// Reference: 2020 Sogang Programming Contest Editorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut safety = vec![i64::MIN; n + 1];
    let mut queue = VecDeque::new();

    for _ in 0..m {
        let password = scan.token::<usize>();

        safety[password] = 0;
        queue.push_back(password);
    }

    let mut ret = 0;

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        for i in 0..20 {
            let next = curr ^ (1 << i);

            if next > n || safety[next] != i64::MIN {
                continue;
            }

            safety[next] = safety[curr] + 1;
            queue.push_back(next);

            ret = ret.max(safety[next]);
        }
    }

    writeln!(out, "{ret}").unwrap();
}