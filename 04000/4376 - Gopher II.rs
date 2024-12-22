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
    matched_gophers: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched_gophers[next] == -1
            || process_dfs(
                graph,
                check,
                matched_gophers,
                matched_gophers[next] as usize,
            )
        {
            matched_gophers[next] = idx as i64;
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let s = scan.line().trim().to_string();

        if s.is_empty() {
            break;
        }

        let chars = s.split_whitespace().collect::<Vec<&str>>();
        let (n, m, s, v) = (
            chars[0].parse::<usize>().unwrap(),
            chars[1].parse::<usize>().unwrap(),
            chars[2].parse::<f64>().unwrap(),
            chars[3].parse::<f64>().unwrap(),
        );
        let mut gophers = vec![(0.0, 0.0); n];
        let mut holes = vec![(0.0, 0.0); m];

        for i in 0..n {
            gophers[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        for i in 0..m {
            holes[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let mut graph = vec![Vec::new(); n];

        for i in 0..n {
            for j in 0..m {
                let dx = gophers[i].0 - holes[j].0;
                let dy = gophers[i].1 - holes[j].1;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= s * v {
                    graph[i].push(j);
                }
            }
        }

        let mut check = vec![false; m];
        let mut matched_gophers = vec![-1; m];
        let mut ret = 0;

        for i in 0..n {
            check.fill(false);

            if process_dfs(&graph, &mut check, &mut matched_gophers, i) {
                ret += 1;
            }
        }

        writeln!(out, "{}", n - ret).unwrap();
    }
}
