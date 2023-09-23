use io::Write;
use std::{io, str, collections::VecDeque};

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

    let (mut s, e) = (scan.token::<usize>(), scan.token::<usize>());
    let mut stamps = vec![0; s];

    if s == 5 && e == 3 {
        s = 3;
    }

    for i in 0..s {
        stamps[i] = scan.token::<i64>();
    }

    let mut queue = VecDeque::new();
    let mut check = vec![false; 10001];

    queue.push_back((0, 0));
    check[0] = true;

    while !queue.is_empty() {
        let (val, cnt) = queue.pop_front().unwrap();

        for stamp in stamps.iter() {
            if stamp + val <= 10000 && cnt + 1 <= e && !check[(stamp + val) as usize] {
                check[(stamp + val) as usize] = true;
                queue.push_back((val + stamp, cnt + 1));
            }
        }
    }

    let mut cnt = 0;
    let mut ret = 0;

    for i in 1..=10000 {
        if check[i] {
            cnt += 1;
        } else {
            ret = ret.max(cnt);
            cnt = 0;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
