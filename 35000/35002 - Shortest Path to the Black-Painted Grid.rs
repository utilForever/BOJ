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

const INF: i64 = i64::MAX / 4;
const BUCKETS: usize = 10;

fn process_dijkstra_reverse(
    neighbors: &Vec<[i64; 4]>,
    cell_type: &Vec<u8>,
    weights: &Vec<u8>,
    limit: i64,
    dest: usize,
    size: usize,
) -> Vec<i64> {
    let mut buckets = vec![Vec::new(); BUCKETS];
    let mut dist = vec![INF; 2 * size];

    buckets[0].push(dest);
    dist[dest] = 0;

    let mut items = 1;
    let mut curr = 0;

    while items > 0 && curr <= limit {
        let idx = (curr as usize) % BUCKETS;

        if buckets[idx].is_empty() {
            curr += 1;
            continue;
        }

        let v = buckets[idx].pop().unwrap();

        items -= 1;

        if dist[v] != curr {
            continue;
        }

        let cell = v >> 1;

        if cell_type[cell] == 0 {
            continue;
        }

        if (v & 1) == 1 {
            let to = v - 1;
            let dist_next = curr + weights[cell] as i64;

            if dist_next <= limit && dist_next < dist[to] {
                dist[to] = dist_next;
                buckets[(dist_next as usize) % BUCKETS].push(to);
                items += 1;
            }
        } else {
            for &neighbor_next in neighbors[cell].iter() {
                if neighbor_next >= 0 {
                    let neighbor_next = neighbor_next as usize;
                    let to = 2 * neighbor_next + 1;
                    let dist_next = curr;

                    if dist_next < dist[to] {
                        dist[to] = dist_next;
                        buckets[(dist_next as usize) % BUCKETS].push(to);
                        items += 1;
                    }
                }
            }
        }
    }

    dist
}

fn process_dijkstra(
    neighbors: &Vec<[i64; 4]>,
    cell_type: &Vec<u8>,
    dist_reverse: &Vec<i64>,
    weights: &mut Vec<u8>,
    d: i64,
    start: usize,
    dest: usize,
    size: usize,
) -> i64 {
    let mut buckets = vec![Vec::new(); BUCKETS];
    let mut dist = vec![INF; 2 * size];

    buckets[0].push(start);
    dist[start] = 0;

    let mut items = 1;
    let mut curr = 0;

    while items > 0 && curr <= d {
        let idx = (curr as usize) % BUCKETS;

        if buckets[idx].is_empty() {
            curr += 1;
            continue;
        }

        let v = buckets[idx].pop().unwrap();

        items -= 1;

        if dist[v] != curr {
            continue;
        }

        if v == dest {
            return curr;
        }

        if dist_reverse[v] == INF {
            continue;
        }

        let cell = v >> 1;

        if cell_type[cell] == 0 {
            continue;
        }

        if (v & 1) == 0 {
            let out = v + 1;

            if cell_type[cell] == 2 && dist_reverse[out] != INF {
                let mut need = d - curr - dist_reverse[out];

                if need > 0 {
                    if need > 9 {
                        need = 9;
                    }

                    let need = need as u8;

                    if weights[cell] < need {
                        weights[cell] = need;
                    }
                }
            }

            let dist_next = curr + weights[cell] as i64;

            if dist_next <= d && dist_next < dist[out] {
                dist[out] = dist_next;
                buckets[(dist_next as usize) % BUCKETS].push(out);
                items += 1;
            }
        } else {
            for &neighbor_next in neighbors[cell].iter() {
                if neighbor_next >= 0 {
                    let neighbor_next = neighbor_next as usize;
                    let to = 2 * neighbor_next;

                    if dist_reverse[to] == INF {
                        continue;
                    }

                    let dist_next = curr;

                    if dist_next < dist[to] {
                        dist[to] = dist_next;
                        buckets[(dist_next as usize) % BUCKETS].push(to);
                        items += 1;
                    }
                }
            }
        }
    }

    INF
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grid = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let (si, sj, ei, ej) = (
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
    );
    let d = scan.token::<i64>();

    let mut cell_type = vec![0u8; n * m];
    let mut digit = vec![0u8; n * m];

    for i in 0..n {
        for j in 0..m {
            let idx = i * m + j;

            match grid[i][j] {
                '#' => {
                    cell_type[idx] = 0;
                    digit[idx] = 0;
                }
                '.' => {
                    cell_type[idx] = 2;
                    digit[idx] = 0;
                }
                '0'..='9' => {
                    cell_type[idx] = 1;
                    digit[idx] = (grid[i][j] as u8) - b'0';
                }
                _ => unreachable!(),
            }
        }
    }

    let idx_start = si * m + sj;
    let idx_end = ei * m + ej;

    let mut neighbors = vec![[-1; 4]; n * m];

    for i in 0..n {
        for j in 0..m {
            let idx = i * m + j;

            if cell_type[idx] == 0 {
                continue;
            }

            if i > 0 {
                let up = idx - m;

                if cell_type[up] != 0 {
                    neighbors[idx][0] = up as i64;
                }
            }

            if i + 1 < n {
                let down = idx + m;

                if cell_type[down] != 0 {
                    neighbors[idx][1] = down as i64;
                }
            }

            if j > 0 {
                let left = idx - 1;

                if cell_type[left] != 0 {
                    neighbors[idx][2] = left as i64;
                }
            }

            if j + 1 < m {
                let right = idx + 1;

                if cell_type[right] != 0 {
                    neighbors[idx][3] = right as i64;
                }
            }
        }
    }

    let dist_reverse =
        process_dijkstra_reverse(&neighbors, &cell_type, &digit, d, 2 * idx_end + 1, n * m);

    if dist_reverse[2 * idx_start] == INF {
        writeln!(out, "-1").unwrap();
        return;
    }

    if dist_reverse[2 * idx_start] != d {
        let dist = process_dijkstra(
            &neighbors,
            &cell_type,
            &dist_reverse,
            &mut digit,
            d,
            2 * idx_start,
            2 * idx_end + 1,
            n * m,
        );

        if dist != d {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    for i in 0..n {
        for j in 0..m {
            let idx = i * m + j;

            grid[i][j] = if cell_type[idx] == 0 {
                '#'
            } else {
                (b'0' + digit[idx]) as char
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{}", grid[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
