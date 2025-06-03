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

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for i in 2..=n {
        let p = scan.token::<usize>();
        graph[p].push(i);
    }

    let mut queue = VecDeque::new();
    let mut freq = vec![0; n + 1];

    queue.push_back(1);
    freq[0] = 1;

    let mut depth = vec![0; n + 1];
    let mut depth_max = 0;

    while let Some(node) = queue.pop_front() {
        let depth_node = depth[node];

        for &next in graph[node].iter() {
            let depth_next = depth_node + 1;

            depth[next] = depth_next;
            freq[depth_next] += 1;
            depth_max = depth_max.max(depth_next);

            queue.push_back(next);
        }
    }

    freq.truncate(depth_max + 1);

    let mut ret = 0;

    for i in 2..=n {
        let mut cnt = 0;
        let mut height = 0;

        while height <= depth_max {
            cnt += freq[height];
            height += i;
        }

        ret = ret.max(cnt);
    }

    writeln!(out, "{ret}").unwrap();
}
