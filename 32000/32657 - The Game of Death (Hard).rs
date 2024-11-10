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

fn check(
    graph: &Vec<Vec<usize>>,
    cache: &mut HashMap<(usize, usize), bool>,
    curr: usize,
    k: usize,
) -> bool {
    if k == 0 {
        return curr != 1;
    }

    if let Some(&ret) = cache.get(&(curr, k)) {
        return ret;
    }

    let ret = if curr == 1 {
        let mut can_win = false;

        for &next in graph[curr].iter() {
            if check(graph, cache, next, k - 1) {
                can_win = true;
                break;
            }
        }

        can_win
    } else {
        let mut can_win = true;

        for &next in graph[curr].iter() {
            if !check(graph, cache, next, k - 1) {
                can_win = false;
                break;
            }
        }
        
        can_win
    };

    cache.insert((curr, k), ret);

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        graph[i].push(l);
        graph[i].push(r);
    }

    for k in 10..=99 {
        if check(&graph, &mut HashMap::new(), 1, k) {
            writeln!(out, "{k}").unwrap();
            return;
        }
    }

    writeln!(out, "-1").unwrap();
}
