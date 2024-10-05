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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process(n: usize) -> String {
    let mut visited = vec![false; n];
    let mut parent = vec![usize::MAX; n];
    let mut digit = vec![' '; n];
    let mut queue = VecDeque::new();

    let idx_start = 1 % n;
    visited[idx_start] = true;
    digit[idx_start] = '1';
    queue.push_back(idx_start);

    while let Some(curr) = queue.pop_front() {
        if curr == 0 {
            let mut num = Vec::new();
            let mut rem = curr;

            for _ in 0..n {
                if digit[rem] == ' ' {
                    break;
                }

                num.push(digit[rem]);

                if parent[rem] == usize::MAX {
                    break;
                }

                rem = parent[rem];
            }

            num.reverse();

            return num.iter().collect::<String>();
        }

        for d in ['0', '1'].iter() {
            let next = (curr * 10 + d.to_digit(10).unwrap() as usize) % n;

            if visited[next] {
                continue;
            }

            visited[next] = true;
            parent[next] = curr;
            digit[next] = *d;
            queue.push_back(next);
        }
    }

    "BRAK".to_string()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        writeln!(out, "{}", process(n)).unwrap();
    }
}
