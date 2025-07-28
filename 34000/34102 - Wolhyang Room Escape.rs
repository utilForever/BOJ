use std::{cmp::Reverse, io, str};

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

#[derive(Clone, PartialEq, Eq)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
    color: Color,
}

impl Edge {
    fn new(u: usize, v: usize, color: Color) -> Self {
        Self { u, v, color }
    }
}

fn digits_of(mut p: i64) -> [u8; 10] {
    let mut ret = [0; 10];

    for i in (0..10).rev() {
        ret[i] = (p % 10) as u8;
        p /= 10;
    }

    ret
}

fn encode(p: i64, n: usize, m: usize, mut edges: Vec<Edge>) -> String {
    let mut deg = vec![0; n];
    let mut incident = vec![Vec::new(); n];

    for (idx, e) in edges.iter().enumerate() {
        deg[e.u] += 1;
        deg[e.v] += 1;

        incident[e.u].push(idx);
        incident[e.v].push(idx);
    }

    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_unstable_by_key(|&val| (Reverse(deg[val]), val));

    let special = order.into_iter().take(10).collect::<Vec<_>>();
    let mut is_special = vec![false; n];

    for &val in special.iter() {
        is_special[val] = true;
    }

    let mut red = vec![0; n];
    let mut green = vec![0; n];
    let password = digits_of(p);

    for (pos, &val) in special.iter().enumerate() {
        let want = pos;

        for &idx in incident[val].iter() {
            if red[val] == want {
                break;
            }

            if edges[idx].color != Color::Blue {
                continue;
            }

            let other = if edges[idx].u == val {
                edges[idx].v
            } else {
                edges[idx].u
            };

            if is_special[other] {
                continue;
            }

            edges[idx].color = Color::Red;
            red[val] += 1;
            red[other] += 1;
        }
    }

    for (pos, &val) in special.iter().enumerate() {
        let want = password[pos] as usize;

        while green[val] % 10 != want {
            for &idx in incident[val].iter() {
                if edges[idx].color != Color::Blue {
                    continue;
                }

                let other = if edges[idx].u == val {
                    edges[idx].v
                } else {
                    edges[idx].u
                };

                if is_special[other] {
                    continue;
                }

                edges[idx].color = Color::Green;
                green[val] += 1;
                green[other] += 1;

                break;
            }
        }
    }

    let mut need = (0..n)
        .map(|v| {
            if is_special[v] {
                0usize
            } else if red[v] >= 10 {
                0
            } else {
                10 - red[v]
            }
        })
        .collect::<Vec<_>>();
    let mut remain = need.iter().sum::<usize>();

    if remain > 0 {
        for (idx, e) in edges.iter_mut().enumerate() {
            if remain == 0 {
                break;
            }

            if e.color != Color::Blue {
                continue;
            }

            if is_special[e.u] || is_special[e.v] {
                continue;
            }

            if need[e.u] == 0 && need[e.v] == 0 {
                continue;
            }

            e.color = Color::Red;

            if need[e.u] > 0 {
                need[e.u] -= 1;
                red[e.u] += 1;
                remain -= 1;
            }

            if need[e.v] > 0 {
                need[e.v] -= 1;
                red[e.v] += 1;
                remain -= 1;
            }
        }
    }

    let ret = edges
        .into_iter()
        .map(|e| match e.color {
            Color::Red => 'R',
            Color::Green => 'G',
            Color::Blue => 'B',
        })
        .collect::<Vec<_>>();

    ret.into_iter().collect::<String>()
}

fn decode(n: usize, edges: &[Edge], colors: &[char]) -> u64 {
    let mut red = vec![0; n];
    let mut green = vec![0; n];

    for (edge, &color) in edges.iter().zip(colors) {
        match color {
            'R' => {
                red[edge.u] += 1;
                red[edge.v] += 1;
            }
            'G' => {
                green[edge.u] += 1;
                green[edge.v] += 1;
            }
            _ => {}
        }
    }

    let mut small = (0..n)
        .filter(|&v| red[v] < 10)
        .map(|v| (red[v], v))
        .collect::<Vec<_>>();

    small.sort_unstable_by_key(|&(r, _)| r);

    let mut ret = 0;

    for &(_, v) in small.iter().take(10) {
        ret = ret * 10 + (green[v] % 10) as u64;
    }

    ret
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let mode = scan.token::<String>();

    if mode == "A" {
        let (p, n, m) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut edges = Vec::with_capacity(m);

        for _ in 0..m {
            let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
            edges.push(Edge::new(u, v, Color::Blue));
        }

        println!("{}", encode(p, n, m, edges));
    } else {
        let (n, m, color) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<String>().chars().collect::<Vec<_>>(),
        );
        let mut edges = Vec::with_capacity(m);

        for _ in 0..m {
            let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
            edges.push(Edge::new(u, v, Color::Blue));
        }

        println!("{}", decode(n, &edges, &color));
    }
}
