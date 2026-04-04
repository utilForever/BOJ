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

    let n = scan.token::<usize>();
    let mut ranges = vec![(0, 0); n];

    for i in 0..n {
        let (l, r) = (scan.token::<i64>(), scan.token::<i64>());
        ranges[i] = (l, r);
    }

    let mut ret_position = vec![0; n];
    let mut ret_priority = vec![0; n];
    let mut stack = Vec::new();

    stack.push(0);

    let mut idx = 1;

    for i in 1..n {
        let mut need = 0;

        while let Some(&j) = stack.last() {
            if ranges[j].1 >= ranges[i].0 {
                break;
            }

            stack.pop();
            need = need.max(ret_position[j] + ranges[j].1);
            ret_priority[j] = idx;
            idx += 1;
        }

        ret_position[i] = if let Some(&j) = stack.last() {
            need.max(ret_position[j] + ranges[i].0)
        } else {
            need
        };

        stack.push(i);
    }

    while let Some(i) = stack.pop() {
        ret_priority[i] = idx;
        idx += 1;
    }

    for i in 0..n {
        writeln!(out, "{} {}", ret_position[i], ret_priority[i]).unwrap();
    }
}
