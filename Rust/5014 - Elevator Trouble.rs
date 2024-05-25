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

    let (f, s, g, u, d) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut visited = vec![false; f + 1];

    let mut queue = VecDeque::new();
    queue.push_back((s, 0));

    while !queue.is_empty() {
        let (curr, count) = queue.pop_front().unwrap();

        if visited[curr] {
            continue;
        }

        visited[curr] = true;

        if curr == g {
            writeln!(out, "{count}").unwrap();
            return;
        }

        if curr + u <= f {
            queue.push_back((curr + u, count + 1));
        }

        if curr >= d + 1 {
            queue.push_back((curr - d, count + 1));
        }
    }

    writeln!(out, "use the stairs").unwrap();
}
