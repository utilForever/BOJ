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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
        let mut build_time = vec![0; n + 1];
        let mut order = vec![Vec::new(); n + 1];
        let mut num_nodes = vec![0; n + 1];

        for i in 1..=n {
            build_time[i] = scan.token::<i64>();
        }

        for _ in 0..k {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
            order[x].push(y);
            num_nodes[y] += 1;
        }

        let w = scan.token::<usize>();
        let mut queue = VecDeque::new();
        let mut build_time_total = vec![0; n + 1];

        for i in 1..=n {
            if num_nodes[i] == 0 {
                queue.push_back(i);
            }

            build_time_total[i] += build_time[i];
        }

        for _ in 1..=n {
            if queue.is_empty() {
                continue;
            }

            let val = queue.pop_front().unwrap();

            for next in order[val].iter() {
                build_time_total[*next] =
                    build_time_total[*next].max(build_time_total[val] + build_time[*next]);
                num_nodes[*next] -= 1;

                if num_nodes[*next] == 0 {
                    queue.push_back(*next);
                }
            }
        }

        writeln!(out, "{}", build_time_total[w]).unwrap();
    }
}
