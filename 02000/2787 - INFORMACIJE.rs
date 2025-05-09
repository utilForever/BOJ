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

#[derive(Clone)]
struct Bitset {
    w: [u64; 4],
}

impl Bitset {
    fn full(n: usize) -> Self {
        let mut w = [u64::MAX; 4];
        let rem = 256 - n;

        if rem > 0 {
            w[3] >>= rem;
        }

        Bitset { w }
    }

    #[inline]
    fn test(&self, idx: usize) -> bool {
        (self.w[idx >> 6] >> (idx & 63)) & 1 == 1
    }

    #[inline]
    fn and_assign(&mut self, other: &Bitset) {
        for i in 0..4 {
            self.w[i] &= other.w[i];
        }
    }

    #[inline]
    fn and_not_assign(&mut self, other: &Bitset) {
        for i in 0..4 {
            self.w[i] &= !other.w[i];
        }
    }

    fn any(&self) -> bool {
        self.w.iter().any(|&x| x != 0)
    }
}

fn range_mask(l: usize, r: usize) -> Bitset {
    let mut m = Bitset { w: [0; 4] };

    for pos in l..=r {
        m.w[pos >> 6] |= 1u64 << (pos & 63);
    }

    m
}

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    check: &mut [bool],
    matched_val: &mut Vec<i64>,
    idx: usize,
) -> bool {
    for &next in graph[idx].iter() {
        if check[next] {
            continue;
        }

        check[next] = true;

        if matched_val[next] == -1
            || process_dfs(graph, check, matched_val, matched_val[next] as usize)
        {
            matched_val[next] = idx as i64;
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
    let mut allowed = vec![Bitset::full(n); n];

    for _ in 0..m {
        let (cmd, mut x, mut y, v) = (
            scan.token::<usize>(),
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
        );

        if x > y {
            std::mem::swap(&mut x, &mut y);
        }

        let mask = range_mask(x, y);

        allowed[v].and_assign(&mask);

        if cmd == 1 {
            for w in (v + 1)..n {
                allowed[w].and_not_assign(&mask);
            }
        } else {
            for w in 0..v {
                allowed[w].and_not_assign(&mask);
            }
        }
    }

    for v in 0..n {
        if !allowed[v].any() {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    let mut graph = vec![Vec::<usize>::new(); n];

    for v in 0..n {
        for pos in 0..n {
            if allowed[v].test(pos) {
                graph[pos].push(v);
            }
        }
    }

    if graph.iter().any(|row| row.is_empty()) {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut check = vec![false; n];
    let mut matched_val = vec![-1; n];

    for pos in 0..n {
        check.fill(false);

        if !process_dfs(&graph, &mut check, &mut matched_val, pos) {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    let mut ret = vec![0; n];

    for (val, &pos) in matched_val.iter().enumerate() {
        if pos != -1 {
            ret[pos as usize] = val + 1;
        }
    }

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
