use io::Write;
use std::{collections::BTreeSet, io, str};

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
    stones: &mut Vec<i64>,
    idxes: &mut Vec<usize>,
    graph: &Vec<Vec<usize>>,
    nimbers: &mut Vec<i64>,
    vertices: &mut Vec<BTreeSet<i64>>,
    curr: usize,
    prev: usize,
) {
    let val = stones[curr];
    let mut set = BTreeSet::new();
    let mut check = vec![false; graph[curr].len() + 1];
    let mut cnt = 0;
    let mut idx_max = 0;
    let mut cnt_vertex = 0;
    let mut nim = 0;

    for &next in graph[curr].iter() {
        cnt += 1;

        if next == prev {
            continue;
        }

        process_dfs(stones, idxes, graph, nimbers, vertices, next, curr);

        nim ^= nimbers[next];

        if vertices[idxes[next]].get(&(val ^ stones[next])).is_some() {
            check[cnt] = true;
        }

        if vertices[idxes[next]].len() > cnt_vertex {
            idx_max = next;
            cnt_vertex = vertices[idxes[next]].len();
        }
    }

    cnt = 0;

    for &next in graph[curr].iter() {
        cnt += 1;

        if next == prev {
            continue;
        }

        if check[cnt] {
            set.insert(nim ^ nimbers[next]);
        }

        if next == idx_max {
            continue;
        }

        let xor_val = stones[idx_max] ^ stones[next];
        let vertices_new: Vec<_> = vertices[idxes[next]]
            .iter()
            .map(|&vertex| xor_val ^ vertex)
            .collect();

        for vertex in vertices_new {
            vertices[idxes[idx_max]].insert(vertex);
        }

        vertices[idxes[next]].clear();
    }

    if stones[curr] == 0 {
        set.insert(nim);
    }

    if idx_max != 0 {
        idxes[curr] = idxes[idx_max];
    }

    stones[curr] ^= stones[idx_max];
    vertices[idxes[curr]].insert(stones[idx_max]);

    if set.is_empty() {
        nimbers[curr] = 0;
    } else {
        let mut ret = true;
        let mut cnt = 0;

        for &elem in set.iter() {
            if elem != cnt {
                nimbers[curr] = cnt;
                ret = false;
                break;
            }

            cnt += 1;
        }

        if ret {
            nimbers[curr] = cnt;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut stones = vec![0; n + 1];
    let mut idxes = vec![0; n + 1];
    let mut nimbers = vec![0; n + 1];
    let mut graph = vec![Vec::new(); n + 1];
    let mut vertices = vec![BTreeSet::new(); n + 1];

    for i in 1..=n {
        stones[i] = scan.token::<i64>();
        idxes[i] = i;
    }

    for _ in 1..n {
        let (p, q) = (scan.token::<usize>(), scan.token::<usize>());
        graph[p].push(q);
        graph[q].push(p);
    }

    process_dfs(
        &mut stones,
        &mut idxes,
        &graph,
        &mut nimbers,
        &mut vertices,
        1,
        0,
    );

    writeln!(
        out,
        "{}",
        if nimbers[1] > 0 {
            "kidw0124"
        } else {
            "eoaud0108"
        }
    )
    .unwrap();
}
