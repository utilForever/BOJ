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

    let (n, p) = (scan.token::<usize>(), scan.token::<usize>());
    let mut people_needed = vec![0; n];
    let mut festival_wanted = vec![Vec::new(); n];
    let mut candidates = vec![(0, 0); p];
    let mut queue = VecDeque::new();

    for i in 0..n {
        people_needed[i] = scan.token::<i64>();
    }

    for i in 0..p {
        let x = scan.token::<usize>();

        if x == 1 {
            let idx = scan.token::<usize>() - 1;
            people_needed[idx] -= 1;

            if people_needed[idx] == 0 {
                queue.push_back(idx);
            }
        } else {
            let mut sum = 0;

            for _ in 0..x {
                let idx = scan.token::<usize>() - 1;

                festival_wanted[idx].push(i);
                sum += idx;
            }

            candidates[i] = (x, sum);
        }
    }

    let mut ret = Vec::new();

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        ret.push(curr);

        for &idx in festival_wanted[curr].iter() {
            candidates[idx].0 -= 1;
            candidates[idx].1 -= curr;

            if candidates[idx].0 == 1 {
                let wanted = candidates[idx].1;
                people_needed[wanted] -= 1;

                if people_needed[wanted] == 0 {
                    queue.push_back(wanted);
                }
            }
        }
    }

    if ret.len() != n {
        writeln!(out, "-1").unwrap();
    } else {
        while !ret.is_empty() {
            writeln!(out, "{}", ret.pop().unwrap() + 1).unwrap();
        }
    }
}
