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

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: i64,
}

impl Edge {
    fn new(u: usize, v: usize, w: i64) -> Self {
        Self { u, v, w }
    }
}

#[derive(Clone, Copy)]
struct Info {
    u: usize,
    v: usize,
    center: i64,
}

impl Info {
    fn new(u: usize, v: usize, center: i64) -> Self {
        Self { u, v, center }
    }
}

fn process_bellman_ford(base: &Vec<Edge>, extra: &Vec<Edge>, n: usize) -> bool {
    let mut dist = vec![0; n];
    let mut edges = Vec::with_capacity(base.len() + extra.len());

    edges.extend_from_slice(base);
    edges.extend_from_slice(extra);

    for i in 0..n {
        let mut updated = false;

        for e in edges.iter() {
            if dist[e.v] > dist[e.u] + e.w {
                dist[e.v] = dist[e.u] + e.w;
                updated = true;

                if i == n - 1 {
                    return false;
                }
            }
        }

        if !updated {
            break;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let w = scan.token::<usize>();
    let s = scan.token::<String>().chars().collect::<Vec<_>>();

    let n = 1usize << (2 * w);
    let len = 1usize << (2 * w + 1);

    if s.len() != len {
        writeln!(out, "no").unwrap();
        return;
    }

    let mut infos = Vec::with_capacity(len);

    for idx in 0..len {
        let u = idx & (n - 1);
        let v = idx >> 1;
        let center = ((idx >> w) & 1) as i64;

        infos.push(Info::new(u, v, center));
    }

    let mut base = Vec::with_capacity(2 * len);

    for &Info { u, v, center } in infos.iter() {
        base.push(Edge::new(u, v, 1 - center));
        base.push(Edge::new(v, u, center));
    }

    let mut extra = Vec::new();

    for (&c, &Info { u, v, center }) in s.iter().zip(infos.iter()) {
        if c == '1' {
            extra.push(Edge::new(u, v, 1 - center));
            extra.push(Edge::new(v, u, center - 1));
        } else {
            extra.push(Edge::new(u, v, -center));
            extra.push(Edge::new(v, u, center));
        }
    }

    if process_bellman_ford(&base, &extra, n) {
        writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
        return;
    }

    for i in (0..len).rev() {
        if s[i] != '0' {
            continue;
        }

        let mut extra = Vec::with_capacity(2 * len);

        for j in 0..i {
            let Info { u, v, center } = infos[j];

            if s[j] == '1' {
                extra.push(Edge::new(u, v, 1 - center));
                extra.push(Edge::new(v, u, center - 1));
            } else {
                extra.push(Edge::new(u, v, -center));
                extra.push(Edge::new(v, u, center));
            }
        }

        let Info { u, v, center } = infos[i];

        extra.push(Edge::new(u, v, 1 - center));
        extra.push(Edge::new(v, u, center - 1));

        if !process_bellman_ford(&base, &extra, n) {
            continue;
        }

        let mut prefix = vec!['0'; len];

        for j in 0..i {
            prefix[j] = s[j];
        }

        prefix[i] = '1';

        let mut check = true;

        for j in i + 1..len {
            let Info { u, v, center } = infos[j];

            extra.push(Edge::new(u, v, -center));
            extra.push(Edge::new(v, u, center));

            if process_bellman_ford(&base, &extra, n) {
                prefix[j] = '0';
                continue;
            }

            extra.pop();
            extra.pop();

            extra.push(Edge::new(u, v, 1 - center));
            extra.push(Edge::new(v, u, center - 1));

            if process_bellman_ford(&base, &extra, n) {
                prefix[j] = '1';
                continue;
            }

            check = false;
            break;
        }

        if check {
            writeln!(out, "{}", prefix.iter().collect::<String>()).unwrap();
            return;
        }
    }

    writeln!(out, "no").unwrap();
}
