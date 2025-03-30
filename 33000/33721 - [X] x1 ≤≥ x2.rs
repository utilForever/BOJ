use io::Write;
use std::{collections::HashSet, io, str};

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

fn process_scc(
    scc_group: &mut Vec<Vec<usize>>,
    scc: &mut Vec<i64>,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<i64>,
    stack: &mut Vec<i64>,
    top: &mut i64,
    cnt: &mut i64,
    node: usize,
) -> i64 {
    *cnt += 1;
    visited[node] = *cnt;

    stack[*top as usize] = node as i64;
    *top += 1;

    let mut ret = visited[node];

    for next in graph[node].iter() {
        if visited[*next as usize] == 0 {
            ret = ret.min(process_scc(
                scc_group,
                scc,
                graph,
                visited,
                stack,
                top,
                cnt,
                *next as usize,
            ));
        } else if scc[*next as usize] == 0 {
            ret = ret.min(visited[*next as usize]);
        }
    }

    if ret == visited[node] {
        scc_group.push(Vec::new());

        loop {
            let now = stack[(*top - 1) as usize];
            *top -= 1;

            scc[now as usize] = scc_group.len() as i64;
            scc_group[(scc[now as usize] - 1) as usize].push(now as usize);

            if now == node as i64 {
                break;
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];
    let mut inequal = HashSet::new();

    for _ in 0..m {
        let (a, op, b) = (
            scan.token::<usize>(),
            scan.token::<String>(),
            scan.token::<usize>(),
        );

        let op_first = op.chars().next().unwrap();

        if a == b && (op.len() < 2 || op_first == '!') {
            writeln!(out, "NO").unwrap();
            return;
        }

        if op_first == '<' || op_first == '=' {
            graph[b].push(a);
        }

        if op_first == '>' || op_first == '=' {
            graph[a].push(b);
        }

        if op.len() == 1 || op_first == '!' {
            inequal.insert((a.min(b), a.max(b)));
        }
    }

    let mut scc_group = Vec::new();
    let mut scc = vec![0; n + 1];
    let mut visited = vec![0; n + 1];
    let mut stack = vec![0; n + 1];
    let mut top = 0;
    let mut cnt = 0;

    for i in 1..=n {
        if visited[i] == 0 {
            process_scc(
                &mut scc_group,
                &mut scc,
                &graph,
                &mut visited,
                &mut stack,
                &mut top,
                &mut cnt,
                i,
            );
        }
    }

    let mut ret = vec![0; n + 1];

    for u in 1..=n {
        for &v in graph[u].iter() {
            if scc[u] == scc[v] && inequal.contains(&(u.min(v), u.max(v))) {
                writeln!(out, "NO").unwrap();
                return;
            }
        }

        ret[u] = scc[u];
    }

    writeln!(out, "YES").unwrap();

    for i in 1..=n {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
