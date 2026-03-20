use io::Write;
use std::{collections::BinaryHeap, io, str};

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

const INF: i64 = i64::MAX / 4;
const DY: [i64; 4] = [-1, 0, 1, 0];
const DX: [i64; 4] = [0, -1, 0, 1];

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize) -> Vec<i64> {
    let mut ret = vec![INF; graph.len()];
    ret[from] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((0, from));

    while !queue.is_empty() {
        let (mut cost_curr, vertex_curr) = queue.pop().unwrap();
        cost_curr *= -1;

        if ret[vertex_curr] < cost_curr {
            continue;
        }

        for info in graph[vertex_curr].iter() {
            let (vertex_next, mut cost_next) = *info;

            cost_next += cost_curr;

            if ret[vertex_next] > cost_next {
                ret[vertex_next] = cost_next;
                queue.push((-cost_next, vertex_next));
            }
        }
    }

    ret
}

fn process_tap_water(
    grid: &Vec<Vec<char>>,
    mut y: i64,
    mut x: i64,
    mut dir: usize,
    n: usize,
    m: usize,
) -> Option<(usize, usize, usize)> {
    for _ in 0..4 {
        dir = (dir + 1) % 4;
        y += DY[dir];
        x += DX[dir];

        if y < 0 || y >= n as i64 || x < 0 || x >= m as i64 {
            return None;
        }

        if grid[y as usize][x as usize] != 'T' {
            return Some((y as usize, x as usize, dir));
        }
    }

    None
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut grid = vec![vec![' '; m]; n];
    let (mut start_y, mut start_x, mut start_dir) = (0, 0, 0);

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;

            if c == '0' || c == '1' || c == '2' || c == '3' {
                grid[i][j] = '.';
                start_y = i;
                start_x = j;
                start_dir = (c as u8 - b'0') as usize;
            }
        }
    }

    let mut cells = Vec::new();
    let mut cells_id = vec![vec![usize::MAX; m]; n];
    let mut cells_sea = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'T' {
                continue;
            }

            let id = cells.len();

            cells.push((i, j));
            cells_id[i][j] = id;

            if grid[i][j] == 'S' {
                cells_sea.push(id);
            }
        }
    }

    let mut graph = vec![Vec::new(); 4 * cells.len()];

    for (idx, &(y, x)) in cells.iter().enumerate() {
        for d in 0..4 {
            let dir_new = (d + 1) % 4;
            let mut edges = Vec::with_capacity(3);

            edges.push((idx * 4 + dir_new, t));

            for dir in [((d + 1) % 4), ((d + 3) % 4)] {
                let y_new = y as i64 + DY[dir];
                let x_new = x as i64 + DX[dir];

                if y_new < 0 || y_new >= n as i64 || x_new < 0 || x_new >= m as i64 {
                    continue;
                }

                let y_new = y_new as usize;
                let x_new = x_new as usize;

                if grid[y_new][x_new] != 'T' {
                    let base_new = cells_id[y_new][x_new];
                    edges.push((base_new * 4 + d, 1));
                } else {
                    if let Some((y, x, dir)) =
                        process_tap_water(&grid, y_new as i64, x_new as i64, d, n, m)
                    {
                        let base_new = cells_id[y][x];
                        edges.push((base_new * 4 + dir, 1));
                    }
                }
            }

            graph[idx * 4 + d] = edges;
        }
    }

    let start_base = cells_id[start_y][start_x];
    let start_state = start_base * 4 + start_dir;

    let dist = process_dijkstra(&graph, start_state);
    let mut ret = INF;

    for &base in cells_sea.iter() {
        for d in 0..4 {
            ret = ret.min(dist[base * 4 + d]);
        }
    }

    writeln!(out, "{}", if ret == INF { -1 } else { ret }).unwrap();
}
