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
    matched: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched[next] == -1 || process_dfs(graph, check, matched, matched[next] as usize) {
            matched[next] = idx as i64;
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k1, k2) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut graph_player = vec![Vec::new(); n];
    let mut graph_enemy = vec![Vec::new(); n];

    for _ in 0..k1 {
        let (i, j) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph_player[i].push(j);
    }

    for _ in 0..k2 {
        let (i, j) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph_enemy[i].push(j);
    }

    let mut check = vec![false; m];
    let mut matched_our = vec![-1; m];
    let mut matched_enemy = vec![-1; m];
    let mut ret_our = 0;
    let mut ret_enemy = 0;

    for i in 0..n {
        check.fill(false);

        if process_dfs(&graph_player, &mut check, &mut matched_our, i) {
            ret_our += 1;
        }
    }

    for i in 0..n {
        check.fill(false);

        if process_dfs(&graph_enemy, &mut check, &mut matched_enemy, i) {
            ret_enemy += 1;
        }
    }

    writeln!(
        out,
        "{}",
        if ret_our < ret_enemy {
            "네 다음 힐딱이"
        } else {
            "그만 알아보자"
        }
    )
    .unwrap();
}
