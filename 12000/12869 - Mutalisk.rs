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
    let mut healths = [0; 3];

    for i in 0..n {
        healths[i] = scan.token::<i64>();
    }

    let mut dp = vec![vec![vec![i64::MAX; 61]; 61]; 61];
    let mut queue = VecDeque::new();

    dp[healths[0] as usize][healths[1] as usize][healths[2] as usize] = 0;
    queue.push_back((healths[0], healths[1], healths[2]));

    let attacks = [
        [9, 3, 1],
        [9, 1, 3],
        [3, 9, 1],
        [3, 1, 9],
        [1, 9, 3],
        [1, 3, 9],
    ];

    while let Some((a, b, c)) = queue.pop_front() {
        let curr = dp[a as usize][b as usize][c as usize];

        for attack in attacks.iter() {
            let a_next = (a - attack[0]).max(0);
            let b_next = (b - attack[1]).max(0);
            let c_next = (c - attack[2]).max(0);

            if dp[a_next as usize][b_next as usize][c_next as usize] > curr + 1 {
                dp[a_next as usize][b_next as usize][c_next as usize] = curr + 1;
                queue.push_back((a_next, b_next, c_next));
            }
        }
    }

    writeln!(out, "{}", dp[0][0][0]).unwrap();
}
