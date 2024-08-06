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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut singers = vec![Vec::new(); n + 1];
    let mut pds = vec![0; n + 1];

    for _ in 0..m {
        let cnt = scan.token::<usize>();
        let mut order = vec![0; n];

        for i in 0..cnt {
            order[i] = scan.token::<usize>();
        }

        for i in 1..cnt {
            pds[order[i]] += 1;
            singers[order[i - 1]].push(order[i]);
        }
    }

    let mut queue = VecDeque::new();

    for i in 1..=n {
        if pds[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut ret = Vec::new();

    while !queue.is_empty() {
        let idx = queue.pop_front().unwrap();
        ret.push(idx);

        for pd in singers[idx].iter() {
            pds[*pd] -= 1;

            if pds[*pd] == 0 {
                queue.push_back(*pd);
            }
        }
    }

    if ret.len() != n {
        writeln!(out, "0").unwrap();
        return;
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
