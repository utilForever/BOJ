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

    let (a, b, n, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut visited = vec![false; 100001];

    let mut queue = VecDeque::new();
    queue.push_back((n, 0));

    while !queue.is_empty() {
        let (pos, cnt) = queue.pop_front().unwrap();

        if pos == m {
            writeln!(out, "{cnt}").unwrap();
            return;
        }

        if pos * a <= 100_000 && !visited[pos * a] {
            visited[pos * a] = true;
            queue.push_back((pos * a, cnt + 1));
        }

        if pos * b <= 100_000 && !visited[pos * b] {
            visited[pos * b] = true;
            queue.push_back((pos * b, cnt + 1));
        }

        if pos as i64 - a as i64 >= 0 && !visited[pos - a] {
            visited[pos - a] = true;
            queue.push_back((pos - a, cnt + 1));
        }

        if pos + a <= 100_000 && !visited[pos + a] {
            visited[pos + a] = true;
            queue.push_back((pos + a, cnt + 1));
        }

        if pos as i64 - b as i64 >= 0 && !visited[pos - b] {
            visited[pos - b] = true;
            queue.push_back((pos - b, cnt + 1));
        }

        if pos + b <= 100_000 && !visited[pos + b] {
            visited[pos + b] = true;
            queue.push_back((pos + b, cnt + 1));
        }

        if pos as i64 - 1 >= 0 && !visited[pos - 1] {
            visited[pos - 1] = true;
            queue.push_back((pos - 1, cnt + 1));
        }

        if pos + 1 <= 100_000 && !visited[pos + 1] {
            visited[pos + 1] = true;
            queue.push_back((pos + 1, cnt + 1));
        }
    }
}
