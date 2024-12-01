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
    let mut evolutions = vec![0; n + 1];

    for i in 1..=n {
        evolutions[i] = scan.token::<usize>();
    }

    let mut visited = vec![false; n + 1];
    let mut cycle = vec![0; n + 1];
    let mut ret = vec![0; 2_000_001];

    for i in 1..=n {
        if visited[i] {
            continue;
        }

        let mut stack = Vec::new();
        let mut idx = i;

        stack.push(idx);

        while !visited[idx] {
            visited[idx] = true;
            idx = evolutions[idx];

            stack.push(idx);
        }

        let prev = stack.pop().unwrap();
        let mut period = vec![prev];
        let mut cnt = 1;
        let mut check = false;

        while !stack.is_empty() {
            if *stack.last().unwrap() == prev {
                check = true;
                break;
            }

            period.push(stack.pop().unwrap());
            cnt += 1;
        }

        if check {
            for &idx in period.iter() {
                cycle[idx] = cnt;
            }

            ret[cnt + 1] += cnt;
        }
    }

    for i in 1..=n {
        if cycle[i] == 0 && cycle[evolutions[i]] != 0 {
            ret[cycle[evolutions[i]] + 1] += 1;
        }
    }

    ret[1] = n;

    for i in (2..=n + 1).rev() {
        let mut idx: usize = 2 * i - 1;

        while idx <= 2_000_000 {
            ret[idx] += ret[i];
            idx += i - 1;
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let c = scan.token::<usize>();
        writeln!(out, "{}", ret[c]).unwrap();
    }
}
