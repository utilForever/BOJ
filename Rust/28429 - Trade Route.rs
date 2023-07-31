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

#[derive(Copy, Clone)]
struct ProfitValues {
    a: i64,
    b: i64,
}

impl ProfitValues {
    fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    fn update(&mut self, value: i64) {
        if value > self.b {
            self.b = value;
        }

        if self.b > self.a {
            let temp = self.a;
            self.a = self.b;
            self.b = temp;
        }
    }
}

static mut DEPTH: usize = 0;

fn process_dfs1(
    graph: &Vec<Vec<usize>>,
    profits: &Vec<i64>,
    depths: &mut Vec<usize>,
    parent: &mut Vec<Vec<usize>>,
    sum_profits: &mut Vec<i64>,
    idx_in: &mut Vec<usize>,
    idx_out: &mut Vec<usize>,
    down: &mut Vec<ProfitValues>,
    curr: usize,
    prev: usize,
) {
    unsafe {
        DEPTH += 1;
    }

    depths[curr] = depths[prev] + 1;
    parent[curr][0] = prev;
    sum_profits[curr] = profits[curr] + sum_profits[prev];
    idx_in[curr] = unsafe { DEPTH };

    for i in 1..20 {
        parent[curr][i] = parent[parent[curr][i - 1]][i - 1];
    }

    down[curr] = ProfitValues::new(profits[curr], profits[curr]);

    for vertex in graph[curr].iter() {
        if *vertex == prev {
            continue;
        }

        process_dfs1(
            graph,
            profits,
            depths,
            parent,
            sum_profits,
            idx_in,
            idx_out,
            down,
            *vertex,
            curr,
        );

        let profit_new = down[*vertex].a + profits[curr];
        down[curr].update(profit_new);
    }

    idx_out[curr] = unsafe { DEPTH };
}

fn process_dfs2(
    graph: &Vec<Vec<usize>>,
    profits: &Vec<i64>,
    up: &mut Vec<i64>,
    down: &mut Vec<ProfitValues>,
    curr: usize,
    prev: usize,
    max: i64,
) {
    let mut temp = ProfitValues::new(-10_i64.pow(18), -10_i64.pow(18));
    temp.update(max + profits[curr]);
    temp.update(profits[curr]);

    up[curr] = temp.a;

    for vertex in graph[curr].iter() {
        if *vertex == prev {
            continue;
        }

        temp.update(down[*vertex].a + profits[curr]);
    }

    for vertex in graph[curr].iter() {
        if *vertex == prev {
            continue;
        }

        if temp.a == down[*vertex].a + profits[curr] {
            process_dfs2(graph, profits, up, down, *vertex, curr, temp.b);
        } else {
            process_dfs2(graph, profits, up, down, *vertex, curr, temp.a);
        }
    }
}

fn find_lca(depths: &Vec<usize>, parent: &Vec<Vec<usize>>, mut a: usize, mut b: usize) -> usize {
    if depths[a] < depths[b] {
        std::mem::swap(&mut a, &mut b);
    }

    for i in (0..=19).rev() {
        if depths[parent[a][i]] >= depths[b] {
            a = parent[a][i];
        }
    }

    if a == b {
        return a;
    }

    for i in (0..=19).rev() {
        if parent[a][i] != parent[b][i] {
            a = parent[a][i];
            b = parent[b][i];
        }
    }

    parent[a][0]
}

fn child(depths: &Vec<usize>, parent: &Vec<Vec<usize>>, a: usize, mut b: usize) -> usize {
    for i in (0..=19).rev() {
        if depths[parent[b][i]] > depths[a] {
            b = parent[b][i];
        }
    }

    b
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut graph = vec![Vec::new(); n + 1];
    let mut profits = vec![0; n + 1];

    for _ in 1..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 1..=n {
        profits[i] = scan.token::<i64>();
    }

    let mut depths = vec![0; n + 1];
    let mut parent = vec![vec![0; 20]; n + 1];
    let mut sum_profits = vec![0; n + 1];
    let mut idx_in = vec![0; n + 1];
    let mut idx_out = vec![0; n + 1];
    let mut up = vec![0; n + 1];
    let mut down = vec![ProfitValues::new(0, 0); n + 1];

    process_dfs1(
        &graph,
        &profits,
        &mut depths,
        &mut parent,
        &mut sum_profits,
        &mut idx_in,
        &mut idx_out,
        &mut down,
        1,
        0,
    );
    process_dfs2(&graph, &profits, &mut up, &mut down, 1, 0, -10_i64.pow(18));

    for _ in 0..q {
        let k = scan.token::<usize>();
        let mut nations = vec![0; k];
        let mut least_ancestor = None;

        for i in 0..k {
            nations[i] = scan.token::<usize>();

            least_ancestor = match least_ancestor {
                Some(ancestor) => Some(find_lca(&depths, &parent, ancestor, nations[i])),
                None => Some(nations[i]),
            };
        }

        let least_ancestor = least_ancestor.unwrap();

        if nations.len() == 1 {
            writeln!(
                out,
                "{}",
                (up[nations[0]] + down[nations[0]].a).max(down[nations[0]].a + down[nations[0]].b)
                    - profits[nations[0]]
            )
            .unwrap();
            continue;
        }

        nations.push(least_ancestor);
        nations.sort_by(|a, b| idx_in[*a].cmp(&idx_in[*b]));

        let mut first = vec![least_ancestor];
        let mut second = vec![least_ancestor];
        let mut is_exist = true;

        for nation in nations.iter() {
            if idx_in[*first.last().unwrap()] <= idx_in[*nation]
                && idx_in[*nation] <= idx_out[*first.last().unwrap()]
            {
                first.push(*nation);
            } else if idx_in[*second.last().unwrap()] <= idx_in[*nation]
                && idx_in[*nation] <= idx_out[*second.last().unwrap()]
            {
                second.push(*nation);
            } else {
                is_exist = false;
                break;
            }
        }

        if !is_exist
            || least_ancestor
                != find_lca(
                    &depths,
                    &parent,
                    *first.last().unwrap(),
                    *second.last().unwrap(),
                )
        {
            writeln!(out, "No").unwrap();
            continue;
        }

        if *first.last().unwrap() == least_ancestor || *second.last().unwrap() == least_ancestor {
            if *first.last().unwrap() == least_ancestor {
                std::mem::swap(&mut first, &mut second);
            }

            let ret = (down[*first.last().unwrap()].a + up[least_ancestor]).max(
                down[*first.last().unwrap()].a
                    + if down[least_ancestor].a
                        == down[child(&depths, &parent, least_ancestor, *first.last().unwrap())].a
                            + profits[least_ancestor]
                    {
                        down[least_ancestor].b
                    } else {
                        down[least_ancestor].a
                    },
            ) + sum_profits[*first.last().unwrap()]
                - profits[*first.last().unwrap()]
                - sum_profits[least_ancestor];
            writeln!(out, "{ret}").unwrap();
        } else {
            let ret = down[*first.last().unwrap()].a
                + down[*second.last().unwrap()].a
                + sum_profits[*first.last().unwrap()]
                + sum_profits[*second.last().unwrap()]
                - 2 * sum_profits[least_ancestor]
                - profits[*first.last().unwrap()]
                - profits[*second.last().unwrap()]
                + profits[least_ancestor];
            writeln!(out, "{ret}").unwrap();
        }
    }
}
