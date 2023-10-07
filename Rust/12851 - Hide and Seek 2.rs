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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut positions = vec![false; 100001];
    let mut ret_time = -1;
    let mut ret_count = 0;

    let mut queue = VecDeque::new();
    queue.push_back((n, 0));

    while !queue.is_empty() {
        let (curr_position, curr_time) = queue.pop_front().unwrap();
        positions[curr_position] = true;

        if curr_position == k {
            if ret_time == -1 {
                ret_time = curr_time;
                ret_count += 1;
            } else if ret_time == curr_time {
                ret_count += 1;
            }

            positions[k] = false;
            continue;
        }

        if curr_position as i64 - 1 >= 0 && !positions[curr_position - 1] {
            queue.push_back((curr_position - 1, curr_time + 1));
        }
        if curr_position + 1 <= 100_000 && !positions[curr_position + 1] {
            queue.push_back((curr_position + 1, curr_time + 1));
        }
        if curr_position * 2 <= 100_000 && !positions[curr_position * 2] {
            queue.push_back((curr_position * 2, curr_time + 1));
        }
    }

    writeln!(out, "{ret_time}").unwrap();
    writeln!(out, "{ret_count}").unwrap();
}
