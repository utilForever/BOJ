use io::Write;
use std::{collections::HashMap, io, str};

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

fn process_dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    arr: &mut Vec<usize>,
    flags_cnt: &Vec<usize>,
    flags: &mut Vec<HashMap<usize, usize>>,
    ret: &mut Vec<usize>,
    curr: usize,
    prev: usize,
) {
    for (next, idx) in graph[curr].iter() {
        if prev == *next {
            continue;
        }

        process_dfs(graph, arr, flags_cnt, flags, ret, *next, curr);

        ret[*idx] = arr[*next];

        if flags[curr].len() < flags[*next].len() {
            flags.swap(curr, *next);
            arr[curr] = arr[*next];
        }

        let flags_next = unsafe { &mut *(&mut flags[*next] as *mut HashMap<usize, usize>) };

        for (key, value) in flags_next.iter() {
            let value_curr = flags[curr].entry(*key).or_insert(0);

            arr[curr] -= (flags_cnt[*key] - *value_curr) * *value_curr;
            *value_curr += value;
            arr[curr] += (flags_cnt[*key] - *value_curr) * *value_curr;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];

    for i in 0..n - 1 {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[a].push((b, i));
        graph[b].push((a, i));
    }

    let mut arr = vec![0; n + 1];
    let mut flags_cnt = vec![0; m + 1];
    let mut flags = vec![HashMap::new(); n + 1];

    for i in 1..=m {
        let c = scan.token::<usize>();
        flags_cnt[i] = c;

        for _ in 0..c {
            let v = scan.token::<usize>();

            arr[v] += c - 1;
            flags[v].insert(i, 1);
        }
    }

    let mut ret = vec![0; n - 1];

    process_dfs(&graph, &mut arr, &flags_cnt, &mut flags, &mut ret, 1, 0);

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
