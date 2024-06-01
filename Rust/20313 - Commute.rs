use io::Write;
use std::{collections::BinaryHeap, io, str};

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

    let (n, m, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut info = vec![(0, 0, 0); m];

    for i in 0..m {
        let (u, v, t) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        info[i] = (u, v, t);
    }

    let k = scan.token::<usize>();
    let mut time_spelled = vec![vec![0; m]; k + 1];

    for i in 1..=k {
        for j in 0..m {
            time_spelled[i][j] = scan.token::<i64>();
        }
    }

    let mut time = vec![vec![i64::MAX; k + 1]; n + 1];
    time[a][0] = 0;

    // (time, next, used_magic)
    let mut queue = BinaryHeap::new();
    queue.push((0, a, 0));

    while !queue.is_empty() {
        let (mut time_curr, vertex_curr, used_magic) = queue.pop().unwrap();
        time_curr *= -1;

        for i in 0..m {
            let (u, v, t) = info[i];

            if u != vertex_curr && v != vertex_curr {
                continue;
            }

            let next = if u == vertex_curr { v } else { u };

            if used_magic < k {
                for j in used_magic + 1..=k {
                    let time_next = time_curr + time_spelled[j][i];

                    if time[next][j] > time_next {
                        time[next][j] = time_next;
                        queue.push((-time_next, next, j));
                    }
                }
            }

            let time_next = time_curr
                + if used_magic == 0 {
                    t
                } else {
                    time_spelled[used_magic][i]
                };

            if time[next][used_magic] > time_next {
                time[next][used_magic] = time_next;
                queue.push((-time_next, next, used_magic));
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 0..=k {
        ret = ret.min(time[b][i]);
    }

    writeln!(out, "{ret}").unwrap();
}
