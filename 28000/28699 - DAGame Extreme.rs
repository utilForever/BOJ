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

fn multiply(x: i64, y: i64, modular: i64) -> i64 {
    (x as i128 * y as i128 % modular as i128) as i64
}

fn pow(x: i64, mut y: i64, p: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % p;

    while y != 0 {
        if y & 1 != 0 {
            ret = multiply(ret, piv, p);
        }

        piv = multiply(piv, piv, p);
        y >>= 1;
    }

    ret
}

fn calculate_mex(check: &Vec<bool>) -> i64 {
    for i in 0..check.len() {
        if !check[i] {
            return i as i64;
        }
    }

    return check.len() as i64;
}

fn process_dfs(
    grundy: &mut Vec<Vec<i64>>,
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    visited_vertices: &mut Vec<usize>,
    idx: usize,
) {
    visited[idx] = true;

    for &next in edges[idx].iter() {
        if !visited[next] {
            process_dfs(grundy, edges, visited, visited_vertices, next);
        }
    }

    let mut check = vec![false; 512];

    for &next in edges[idx].iter() {
        check[grundy[next][next] as usize] = true;
    }

    grundy[idx][idx] = calculate_mex(&check);

    for vertex in visited_vertices.iter() {
        check.clear();
        check.resize(512, false);

        for &next in edges[idx].iter() {
            check[grundy[*vertex][next] as usize] = true;
        }

        for &next in edges[*vertex].iter() {
            check[grundy[idx][next] as usize] = true;
        }

        grundy[idx][*vertex] = calculate_mex(&check);
        grundy[*vertex][idx] = grundy[idx][*vertex];
    }

    visited_vertices.push(idx);
}

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut grundy = vec![vec![-1; n]; n];
    let mut edges = vec![Vec::new(); n];
    let mut nums = vec![0; 256];
    let mut nums_xor1 = vec![vec![0; 256]; 256];
    let mut nums_xor2 = vec![vec![0; 512]; 256];
    let mut sum_xor = vec![0; 256];
    let mut visited: Vec<bool> = vec![false; n];
    let mut visited_vertices = Vec::new();

    for _ in 0..m {
        let (p, q) = (scan.token::<usize>(), scan.token::<usize>());
        edges[p].push(q);
    }

    for i in 0..n {
        if !visited[i] {
            process_dfs(&mut grundy, &edges, &mut visited, &mut visited_vertices, i);
        }
    }

    for i in 0..256 {
        nums[i] = scan.token::<usize>();
    }

    let k = scan.token::<usize>();
    let mut colors = vec![Vec::new(); n];

    for _ in 0..k {
        let (c, e) = (scan.token::<usize>(), scan.token::<usize>());
        colors[c].push(e);
    }

    for i in 0..256 {
        for j in 0..256 {
            nums_xor1[i ^ j][nums[i] ^ nums[j]] += 1;
        }
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..256 {
                nums_xor2[k][grundy[i][j] as usize] += nums_xor1[i ^ j][k];
                sum_xor[k] += nums_xor1[i ^ j][k];
            }
        }
    }

    let mut ret = vec![0; 512];
    let mut is_initialized = false;

    for i in 0..n {
        if colors[i].is_empty() {
            continue;
        }

        let mut check = vec![0; 512];

        if colors[i].len() == 1 {
            for j in 0..n {
                check[grundy[j][j] as usize] += 1;
            }

            let p = pow(n as i64, MOD - 2, MOD);

            for j in 0..512 {
                check[j] = (check[j] * p) % MOD;
            }
        } else {
            let p = pow(sum_xor[colors[i][0] ^ colors[i][1]], MOD - 2, MOD);

            for j in 0..512 {
                check[j] = (nums_xor2[colors[i][0] ^ colors[i][1]][j] * p) % MOD;
            }
        }

        if is_initialized {
            let mut ret_new = vec![0; 512];

            for j in 0..512 {
                for k in 0..512 {
                    ret_new[j ^ k] = (ret_new[j ^ k] + ret[j] * check[k]) % MOD;
                }
            }

            ret = ret_new;
        } else {
            ret = check.clone();
            is_initialized = true;
        }
    }

    writeln!(out, "{}", (1 - ret[0] + MOD) % MOD).unwrap();
}
