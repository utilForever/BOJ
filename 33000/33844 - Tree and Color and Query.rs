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

const N_MAX: usize = 50000;
const N_BIT_MAX: usize = 17;

static mut DIFF: [i64; N_MAX + 1] = [0; N_MAX + 1];
static mut DIFF_ODD: [i64; 2 * (N_MAX + 1)] = [0; 2 * (N_MAX + 1)];

static mut ETT_START: [usize; N_MAX + 1] = [0; N_MAX + 1];
static mut ETT_END: [usize; N_MAX + 1] = [0; N_MAX + 1];
static mut UP: [[usize; N_MAX + 1]; N_BIT_MAX] = [[0; N_MAX + 1]; N_BIT_MAX];
static mut DEPTH: [usize; N_MAX + 1] = [0; N_MAX + 1];

static mut VISITED: [bool; N_MAX + 1] = [false; N_MAX + 1];
static mut MAIN_CNT: [i64; N_MAX + 1] = [0; N_MAX + 1];
static mut MAIN_SUM: i64 = 0;
static mut SUB_CNT: [i64; N_MAX + 1] = [0; N_MAX + 1];
static mut SUB_SUM: i64 = 0;

unsafe fn process_ett(graph: &Vec<Vec<usize>>, root: usize) -> Vec<usize> {
    let mut stack = vec![(root, 0, false)];
    let mut order = Vec::new();

    while let Some((node, parent, visited)) = stack.pop() {
        if visited {
            ETT_END[node] = order.len();
            order.push(node);
        } else {
            UP[0][node] = parent;

            for i in 1..N_BIT_MAX {
                UP[i][node] = UP[i - 1][UP[i - 1][node]];
            }

            ETT_START[node] = order.len();
            order.push(node);
            stack.push((node, parent, true));

            for &next in graph[node].iter().rev() {
                if next != parent {
                    DEPTH[next] = DEPTH[node] + 1;
                    stack.push((next, node, false));
                }
            }
        }
    }

    order
}

unsafe fn lca(mut u: usize, mut v: usize) -> usize {
    if DEPTH[u] < DEPTH[v] {
        std::mem::swap(&mut u, &mut v);
    }

    let diff = DEPTH[u] - DEPTH[v];

    for i in (0..N_BIT_MAX).rev() {
        if diff & (1 << i) != 0 {
            u = UP[i][u];
        }
    }

    if u == v {
        return u;
    }

    for i in (0..N_BIT_MAX).rev() {
        if UP[i][u] != UP[i][v] {
            u = UP[i][u];
            v = UP[i][v];
        }
    }

    UP[0][u]
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum QueryType {
    Path,
    Subtree,
}

#[derive(Debug, Clone)]
struct Query {
    idx: usize,
    left: usize,
    right: usize,
    time: usize,
    extra: Option<usize>,
    r#type: QueryType,
}

impl Query {
    fn new(
        idx: usize,
        left: usize,
        right: usize,
        time: usize,
        extra: Option<usize>,
        r#type: QueryType,
    ) -> Self {
        Self {
            idx,
            left,
            right,
            time,
            extra,
            r#type,
        }
    }
}

#[derive(Clone)]
struct Update {
    vertex: usize,
    color_old: usize,
    color_new: usize,
}

impl Update {
    fn new(vertex: usize, color_old: usize, color_new: usize) -> Self {
        Self {
            vertex,
            color_old,
            color_new,
        }
    }
}

#[inline]
unsafe fn add_main(c: usize) {
    MAIN_CNT[c] += 1;
    MAIN_SUM += c as i64 * DIFF[MAIN_CNT[c] as usize];
}

#[inline]
unsafe fn remove_main(c: usize) {
    MAIN_SUM -= c as i64 * DIFF[MAIN_CNT[c] as usize];
    MAIN_CNT[c] -= 1;
}

#[inline]
unsafe fn add_sub(c: usize) {
    SUB_CNT[c] += 1;
    SUB_SUM += c as i64 * DIFF_ODD[SUB_CNT[c] as usize];
}

#[inline]
unsafe fn remove_sub(c: usize) {
    SUB_SUM -= c as i64 * DIFF_ODD[SUB_CNT[c] as usize];
    SUB_CNT[c] -= 1;
}

#[inline]
unsafe fn flip_node(vertex: usize, colors_curr: &Vec<usize>) {
    let color = colors_curr[vertex];

    if !VISITED[vertex] {
        add_main(color);
        VISITED[vertex] = true;
    } else {
        remove_main(color);
        VISITED[vertex] = false;
    }
}

#[inline]
unsafe fn touch(order: &Vec<usize>, colors_curr: &Vec<usize>, idx: usize, is_add: bool) {
    let vertex = order[idx];

    flip_node(vertex, colors_curr);

    if is_add {
        add_sub(colors_curr[vertex]);
    } else {
        remove_sub(colors_curr[vertex]);
    }
}

unsafe fn apply_update(
    updates: &Vec<Update>,
    order: &Vec<usize>,
    colors_curr: &mut Vec<usize>,
    idx: usize,
    is_forward: bool,
    left: usize,
    right: usize,
) {
    let update = &updates[idx];
    let (vertex, color_new) = if is_forward {
        (update.vertex, update.color_new)
    } else {
        (update.vertex, update.color_old)
    };

    for &pos in [ETT_START[vertex], ETT_END[vertex]].iter() {
        if left <= pos && right >= pos {
            touch(order, colors_curr, pos, false);
        }
    }

    colors_curr[vertex] = color_new;

    for &pos in [ETT_START[vertex], ETT_END[vertex]].iter() {
        if left <= pos && right >= pos {
            touch(order, colors_curr, pos, true);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut colors = vec![0; n + 1];

    for i in 1..=n {
        colors[i] = scan.token::<usize>();
    }

    let mut prev = 0;

    unsafe {
        for i in 1..=n {
            let curr = scan.token::<i64>();

            DIFF[i] = curr - prev;
            prev = curr;
        }

        for i in 1..=n {
            DIFF_ODD[2 * i - 1] = DIFF[i];
        }
    }

    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    let order = unsafe { process_ett(&graph, 1) };
    let mut queries = Vec::new();
    let mut updates = Vec::new();

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let x = scan.token::<usize>();
            queries.push(Query::new(
                queries.len(),
                unsafe { ETT_START[x] },
                unsafe { ETT_END[x] },
                updates.len(),
                None,
                QueryType::Subtree,
            ));
        } else if command == 2 {
            let (mut x, mut y) = (scan.token::<usize>(), scan.token::<usize>());

            unsafe {
                if ETT_START[x] > ETT_START[y] {
                    std::mem::swap(&mut x, &mut y);
                }

                let z = lca(x, y);

                if z == x {
                    queries.push(Query::new(
                        queries.len(),
                        ETT_START[x],
                        ETT_START[y],
                        updates.len(),
                        None,
                        QueryType::Path,
                    ));
                } else {
                    queries.push(Query::new(
                        queries.len(),
                        ETT_END[x],
                        ETT_START[y],
                        updates.len(),
                        Some(z),
                        QueryType::Path,
                    ));
                }
            }
        } else {
            let (x, z) = (scan.token::<usize>(), scan.token::<usize>());
            updates.push(Update::new(x, 0, z));
        }
    }

    {
        let mut color_clone = colors.clone();

        for update in updates.iter_mut() {
            update.color_old = color_clone[update.vertex];
            color_clone[update.vertex] = update.color_new;
        }
    }

    let block = ((2 * n) as f64).powf(2.0 / 3.0) as usize + 1;

    queries.sort_by(|a, b| {
        let block_left_a = a.left / block;
        let block_left_b = b.left / block;

        if block_left_a != block_left_b {
            return block_left_a.cmp(&block_left_b);
        }

        let block_right_a = a.right / block;
        let block_right_b = b.right / block;

        if block_right_a != block_right_b {
            if block_left_a % 2 == 0 {
                return block_right_a.cmp(&block_right_b);
            } else {
                return block_right_b.cmp(&block_right_a);
            }
        }

        a.time.cmp(&b.time)
    });

    let mut colors_curr = colors.clone();
    let mut left: isize = 1;
    let mut right: isize = 0;
    let mut time = 0;
    let mut ret = vec![0; queries.len()];

    unsafe {
        for query in queries {
            while (left as usize) > query.left {
                left -= 1;
                touch(&order, &colors_curr, left as usize, true);
            }

            while (right as usize) < query.right {
                right += 1;
                touch(&order, &colors_curr, right as usize, true);
            }

            while (right as usize) > query.right {
                touch(&order, &colors_curr, right as usize, false);
                right -= 1;
            }

            while (left as usize) < query.left {
                touch(&order, &colors_curr, left as usize, false);
                left += 1;
            }

            while time < query.time {
                apply_update(
                    &updates,
                    &order,
                    &mut colors_curr,
                    time,
                    true,
                    left as usize,
                    right as usize,
                );
                time += 1;
            }

            while time > query.time {
                time -= 1;
                apply_update(
                    &updates,
                    &order,
                    &mut colors_curr,
                    time,
                    false,
                    left as usize,
                    right as usize,
                );
            }

            if let Some(x) = query.extra {
                flip_node(x, &colors_curr);
                ret[query.idx] = MAIN_SUM;
                flip_node(x, &colors_curr);
            } else {
                ret[query.idx] = if query.r#type == QueryType::Path {
                    MAIN_SUM
                } else {
                    SUB_SUM
                };
            }
        }
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
