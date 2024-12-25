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

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    matched_collars: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched_collars[next] == -1
            || process_dfs(
                graph,
                check,
                matched_collars,
                matched_collars[next] as usize,
            )
        {
            matched_collars[next] = idx as i64;
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut shirts = vec![0; n];
    let mut collars = vec![0; m];

    for i in 0..n {
        shirts[i] = scan.token::<i64>();
    }

    for i in 0..m {
        collars[i] = scan.token::<i64>();
    }

    let mut graph = vec![Vec::new(); n];

    for (idx_shirt, &shirt) in shirts.iter().enumerate() {
        let min1 = (shirt as f64 / 2.0).ceil() as i64;
        let max1 = ((3 * shirt) as f64 / 4.0).floor() as i64;

        let min2 = shirt;
        let max2 = ((5 * shirt) as f64 / 4.0).floor() as i64;

        for (idx_collar, &collar) in collars.iter().enumerate() {
            if (min1 <= collar && collar <= max1) || (min2 <= collar && collar <= max2) {
                graph[idx_shirt].push(idx_collar);
            }
        }
    }

    let mut check = vec![false; m];
    let mut matched_collars = vec![-1; m];
    let mut ret = 0;

    for i in 0..n {
        check.fill(false);

        if process_dfs(&graph, &mut check, &mut matched_collars, i) {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
