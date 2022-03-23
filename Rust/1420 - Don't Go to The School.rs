use io::Write;
use std::{cell::RefCell, collections::VecDeque, io, rc::Rc, str};

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

struct Edge {
    dest: Option<Rc<RefCell<Edge>>>,
    to: usize,
    capacity: i64,
}

impl Edge {
    fn new(to: usize, capacity: i64) -> Self {
        Self {
            dest: None,
            to,
            capacity,
        }
    }
}

struct MCMF {
    graph: Vec<Vec<Rc<RefCell<Edge>>>>,
    source: usize,
    sink: usize,
}

impl MCMF {
    fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n as usize],
            source,
            sink,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, capacity: i64) {
        let orig = Rc::new(RefCell::new(Edge::new(v, capacity)));
        let dest = Rc::new(RefCell::new(Edge::new(u, 0)));

        orig.as_ref().borrow_mut().dest = Some(dest.clone());
        dest.as_ref().borrow_mut().dest = Some(orig.clone());

        self.graph[u].push(orig);
        self.graph[v].push(dest);
    }

    fn process_dfs(&mut self) -> i64 {
        let mut check = vec![false; self.graph.len()];
        let mut from = vec![(-1, -1); self.graph.len()];
        let mut queue = VecDeque::new();

        queue.push_back(self.source);
        check[self.source] = true;

        while !queue.is_empty() {
            let val = queue.pop_front().unwrap();

            for i in 0..self.graph[val].len() {
                if self.graph[val][i].borrow().capacity > 0
                    && !check[self.graph[val][i].borrow().to]
                {
                    queue.push_back(self.graph[val][i].borrow().to);
                    check[self.graph[val][i].borrow().to] = true;
                    from[self.graph[val][i].borrow().to] = (val as i64, i as i64);
                }
            }
        }

        if !check[self.sink] {
            return 0;
        }

        let mut x = self.sink;
        let mut capacity = self.graph[from[x].0 as usize][from[x].1 as usize]
            .as_ref()
            .borrow()
            .capacity;

        while from[x].0 != -1 {
            if capacity
                > self.graph[from[x].0 as usize][from[x].1 as usize]
                    .as_ref()
                    .borrow()
                    .capacity
            {
                capacity = self.graph[from[x].0 as usize][from[x].1 as usize]
                    .as_ref()
                    .borrow()
                    .capacity;
            }

            x = from[x].0 as usize;
        }

        x = self.sink;

        while from[x].0 != -1 {
            let edge = &mut self.graph[from[x].0 as usize][from[x].1 as usize].borrow_mut();
            edge.capacity -= capacity;
            unsafe {
                (*edge.dest.as_ref().unwrap().as_ref().as_ptr()).capacity += capacity;
            }

            x = from[x].0 as usize;
        }

        capacity
    }

    fn get_flow(&mut self) -> i64 {
        let mut total_flow = 0;

        loop {
            let res = self.process_dfs();

            if res == 0 {
                break;
            }

            total_flow += res;
        }

        total_flow
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut city = vec![vec!['0'; m]; n];

    for i in 0..n {
        let s = scan.token::<String>();
        let mut j = 0;

        for ch in s.chars() {
            city[i][j] = ch;
            j += 1;
        }
    }

    let mut x_start = 0;
    let mut x_end = 0;
    let mut y_start = 0;
    let mut y_end = 0;

    for i in 0..n {
        for j in 0..m {
            if city[i][j] == 'K' {
                y_start = i;
                x_start = j;
            } else if city[i][j] == 'H' {
                y_end = i;
                x_end = j;
            }
        }
    }

    let mut mcmf = MCMF::new(
        (n * m) * 2,
        (y_start * m + x_start) * 2 + 1,
        (y_end * m + x_end) * 2,
    );

    for i in 0..n {
        for j in 0..m {
            if city[i][j] == '#' {
                continue;
            }

            mcmf.add_edge((i * m + j) * 2, (i * m + j) * 2 + 1, 1);

            for k in 0..4 {
                let y_new = i as i64 + dy[k];
                let x_new = j as i64 + dx[k];

                if y_new >= 0 && y_new < n as i64 && x_new >= 0 && x_new < m as i64 {
                    if city[y_new as usize][x_new as usize] != '#' {
                        mcmf.add_edge(
                            (i * m + j) * 2 + 1,
                            (y_new as usize * m + x_new as usize) * 2,
                            1_000_000,
                        );
                    }
                }
            }
        }
    }

    let mut ans = mcmf.get_flow();
    if ans >= 1_000_000 {
        ans = -1;
    }

    writeln!(out, "{}", ans).unwrap();
}
