use io::Write;
use std::{cmp, io, str};

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

static mut DFS_COUNT: usize = 0;
static mut S_COUNT: usize = 0;

#[derive(Default, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn calculate_ccw(p1: &Point, p2: &Point, p3: &Point) -> i64 {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);

    let res = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
    if res > 0 {
        1
    } else if res < 0 {
        -1
    } else {
        0
    }
}

fn is_intersect(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    let mut p = calculate_ccw(p1, p2, p3);
    let mut q = calculate_ccw(p1, p2, p4);

    if p != 0 && q != 0 && p == q {
        return false;
    }

    p = calculate_ccw(p3, p4, p1);
    q = calculate_ccw(p3, p4, p2);

    if p != 0 && q != 0 && p == q {
        return false;
    }

    true
}

fn get_opposite(n: usize) -> usize {
    if n % 2 == 1 {
        n - 1
    } else {
        n + 1
    }
}

unsafe fn process_dfs(
    adj: &Vec<Vec<usize>>,
    stack: &mut Vec<usize>,
    is_finished: &mut Vec<bool>,
    dfs_n: &mut Vec<usize>,
    s_n: &mut Vec<usize>,
    n: usize,
    cur: usize,
) -> usize {
    DFS_COUNT += 1;
    dfs_n[cur] = DFS_COUNT;

    stack.push(cur);

    let mut result = dfs_n[cur];

    for next in adj[cur].iter() {
        if dfs_n[*next] == 0 {
            result = cmp::min(
                result,
                process_dfs(adj, stack, is_finished, dfs_n, s_n, n, *next),
            );
        } else if !is_finished[*next] {
            result = cmp::min(result, dfs_n[*next]);
        }
    }

    if result >= dfs_n[cur] {
        s_n[cur] = S_COUNT;
        S_COUNT += 1;

        loop {
            let family = stack.pop().unwrap();
            is_finished[family] = true;

            if family == cur {
                break;
            }

            s_n[family] = s_n[cur];
        }
    }

    result
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut p1 = vec![Point::default(); n * 3];
    let mut p2 = vec![Point::default(); n * 3];
    let mut adj = vec![Vec::new(); n * 6];

    for i in 0..(n * 3) {
        p1[i] = Point::new(scan.token(), scan.token());
        p2[i] = Point::new(scan.token(), scan.token());

        let k1 = i / 3;
        let k2 = i % 3;
        let mut p = k1 * 6 + k2 * 2;
        let mut q = k1 * 6 + (k2 + 1) % 3 * 2;

        adj[get_opposite(q)].push(p);
        adj[get_opposite(p)].push(q);

        for j in 0..i {
            if is_intersect(&p1[i], &p2[i], &p1[j], &p2[j]) {
                p = i * 2 + 1;
                q = j * 2 + 1;

                adj[get_opposite(q)].push(p);
                adj[get_opposite(p)].push(q);
            }
        }
    }

    let mut stack = Vec::new();
    let mut is_finished = vec![false; n * 6];
    let mut dfs_n = vec![0; n * 6];
    let mut s_n = vec![0; n * 6];

    for i in 0..(n * 6) {
        if dfs_n[i] == 0 {
            unsafe {
                process_dfs(
                    &adj,
                    &mut stack,
                    &mut is_finished,
                    &mut dfs_n,
                    &mut s_n,
                    n,
                    i,
                );
            }
        }
    }

    let mut is_possible = true;

    for i in 0..(n * 3) {
        if s_n[i * 2] == s_n[i * 2 + 1] {
            is_possible = false;
            break;
        }
    }

    if !is_possible {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut result: Vec<i32> = vec![-1; n * 3];
    let mut data = vec![(0, 0); n * 6];

    for i in 0..(n * 6) {
        data[i] = (s_n[i] as i32, i as i32);
    }

    data.sort();

    for i in (0..=(n * 6 - 1)).rev() {
        let val = data[i].1 as usize;

        if result[val / 2] == -1 {
            result[val / 2] = if val as i32 % 2 == 0 { 1 } else { 0 };
        }
    }

    let mut stick = Vec::new();

    for i in 0..(n * 3) {
        if result[i] != 0 {
            stick.push(i + 1);
        }
    }

    writeln!(out, "{}", stick.len()).unwrap();

    for i in 0..stick.len() {
        write!(out, "{} ", stick[i]).unwrap();
    }

    writeln!(out).unwrap();
}
