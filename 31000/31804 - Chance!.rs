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

    let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ret = vec![vec![0; 2]; 1_000_001];

    let mut queue = VecDeque::new();
    queue.push_back((a, 0));

    while !queue.is_empty() {
        let (num, use_chance) = queue.pop_front().unwrap();

        if num == b {
            writeln!(out, "{}", ret[num][use_chance]).unwrap();
            break;
        }

        if num + 1 <= 1_000_000 && ret[num + 1][use_chance] == 0 {
            queue.push_back((num + 1, use_chance));
            ret[num + 1][use_chance] = ret[num][use_chance] + 1;
        }

        if num * 2 <= 1_000_000 && ret[num * 2][use_chance] == 0 {
            queue.push_back((num * 2, use_chance));
            ret[num * 2][use_chance] = ret[num][use_chance] + 1;
        }

        if num * 10 <= 1_000_000 && ret[num * 10][1] == 0 && use_chance == 0 {
            queue.push_back((num * 10, 1));
            ret[num * 10][1] = ret[num][use_chance] + 1;
        }
    }
}
