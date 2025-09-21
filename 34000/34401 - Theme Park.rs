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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, p, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut groups = vec![(0, 0); n];

    for i in 0..n {
        groups[i] = (scan.token::<i64>(), scan.token::<usize>());
    }

    groups.sort_unstable();

    let mut buf = vec![Vec::new(); k + 1];
    let mut head = vec![0; k + 1];

    let mut idx = 0;
    let mut cnt_completed = 0;
    let mut time = 0;
    let mut ret = 0;

    while cnt_completed < n {
        while idx < n && groups[idx].0 <= time {
            let (t, a) = groups[idx];

            buf[a].push(t);
            idx += 1;
        }

        let mut capacity = k;

        while capacity > 0 {
            let mut best_size = 0;
            let mut best_time = i64::MAX;

            for i in 1..=capacity {
                if head[i] < buf[i].len() {
                    let t_front = buf[i][head[i]];

                    if t_front < best_time {
                        best_time = t_front;
                        best_size = i;
                    }
                }
            }

            if best_size == 0 {
                break;
            }

            head[best_size] += 1;
            cnt_completed += 1;
            capacity -= best_size;
            ret += time - best_time;
        }

        time += p;
    }

    writeln!(out, "{ret}").unwrap();
}
