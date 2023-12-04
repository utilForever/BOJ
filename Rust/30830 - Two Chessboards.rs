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

    let n = scan.token::<usize>();
    let mut rooks = vec![(0, 0); 2 * n + 1];
    let mut rows = vec![Vec::new(); n + 1];
    let mut cols = vec![Vec::new(); n + 1];

    for i in 1..=2 * n {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        rooks[i] = (x, y);
        rows[x].push(i);
        cols[y].push(i);
    }

    for i in 1..=n {
        if rows[i].len() != 2 || cols[i].len() != 2 {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    let mut visited = vec![false; 2 * n + 1];
    let mut ret = 0;

    for i in 1..=2 * n {
        if visited[i] {
            continue;
        }

        let mut queue = VecDeque::new();
        let mut cnt = 0;
        let mut total = 0;

        visited[i] = true;
        queue.push_back((i, 0));

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let x_next = rows[rooks[x].0][0] + rows[rooks[x].0][1] - x;
            let y_next = cols[rooks[x].1][0] + cols[rooks[x].1][1] - x;

            if (x <= n && y == 1) || (x > n && y == 0) {
                cnt += 1;
            }

            total += 1;

            if !visited[x_next] {
                visited[x_next] = true;
                queue.push_back((x_next, y ^ 1));
            }

            if !visited[y_next] {
                visited[y_next] = true;
                queue.push_back((y_next, y ^ 1));
            }
        }

        ret += cnt.min(total - cnt);
    }

    writeln!(out, "{}", ret / 2).unwrap();
}
