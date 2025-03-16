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

    let (n, t) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut cats = vec![0; t];

    for i in 0..t {
        cats[i] = scan.token::<usize>() - 1;
    }

    let mut dp = vec![i64::MIN; n];

    for i in 0..n {
        dp[i] = if cats[0] == i { 1 } else { 0 };
    }

    for time in 1..t {
        let mut dp_new = vec![i64::MIN; n];

        for i in 0..n {
            if dp[i] == i64::MIN {
                continue;
            }

            dp_new[i] = dp_new[i].max(dp[i] + if i == cats[time] { 1 } else { 0 });

            for &j in graph[i].iter() {
                dp_new[j] = dp_new[j].max(dp[i] + if j == cats[time] { 1 } else { 0 });
            }
        }

        dp = dp_new;
    }

    writeln!(out, "{}", dp.iter().max().unwrap()).unwrap();
}
