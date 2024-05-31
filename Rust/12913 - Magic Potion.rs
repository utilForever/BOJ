use io::Write;
use std::{cmp::Ordering, collections::BinaryHeap, io, str};

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

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n];
    let mut time = vec![vec![f64::MAX; k + 1]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            if i == j {
                continue;
            }

            graph[i].push((j, c as u8 - b'0'));
        }
    }

    time[0][0] = 0.0;

    // (time, next, used_potion)
    let mut queue = BinaryHeap::new();
    queue.push((MinNonNan(0.0), 0, 0));

    while !queue.is_empty() {
        let (mut time_curr, vertex_curr, used_potion) = queue.pop().unwrap();
        time_curr.0 *= -1.0;

        for &(next, cost) in graph[vertex_curr].iter() {
            if used_potion < k {
                let time_next = time_curr.0 + cost as f64 / 2.0;

                if time[next][used_potion + 1] > time_next {
                    time[next][used_potion + 1] = time_next;
                    queue.push((MinNonNan(-time_next), next, used_potion + 1));
                }
            }

            let time_next = time_curr.0 + cost as f64;

            if time[next][used_potion] > time_next {
                time[next][used_potion] = time_next;
                queue.push((MinNonNan(-time_next), next, used_potion));
            }
        }
    }

    let mut ret = f64::MAX;

    for i in 0..=k {
        ret = ret.min(time[1][i]);
    }

    writeln!(out, "{:.1}", ret).unwrap();
}
