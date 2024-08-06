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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut connect = vec![(0, 0); n + 1];
    let mut visited = vec![false; n + 1];

    for _ in 1..=n {
        let (a, b, p) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        connect[a] = (b, p);
    }

    let calculate_dp = |list: &Vec<i64>| -> i64 {
        let mut dp = vec![vec![0; 2]; list.len()];
        dp[0][0] = 0;
        dp[0][1] = list[0];

        for i in 1..list.len() {
            dp[i][0] = dp[i - 1][1];
            dp[i][1] = dp[i - 1][0].min(dp[i - 1][1]) + list[i];
        }

        dp[list.len() - 1][1]
    };
    
    let mut ret = 0;

    for i in 1..=n {
        if connect[i].0 == i {
            ret += connect[i].1;
            continue;
        }

        if visited[i] {
            continue;
        }

        let mut idx = i;
        let mut list = Vec::new();

        while !visited[idx] {
            visited[idx] = true;
            list.push(connect[idx].1);
            idx = connect[idx].0;
        }

        let cost = calculate_dp(&list);

        list.push(list[0]);
        list.remove(0);

        let cost = cost.min(calculate_dp(&list));
        ret += cost;
    }

    writeln!(out, "{ret}").unwrap();
}
