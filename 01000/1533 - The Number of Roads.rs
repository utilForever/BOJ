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

const MOD: i64 = 1_000_003;

fn matrix_multiply(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, n: usize) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][j] = (ret[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }

    ret
}

fn matrix_pow(mut a: Vec<Vec<i64>>, mut exp: u64, n: usize) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![0; n]; n];

    for i in 0..n {
        ret[i][i] = 1;
    }

    while exp > 0 {
        if exp % 2 == 1 {
            ret = matrix_multiply(&ret, &a, n);
        }

        a = matrix_multiply(&a, &a, n);
        exp /= 2;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, s, e, t) = (
        scan.token::<usize>(),
        scan.token::<usize>() - 1,
        scan.token::<usize>() - 1,
        scan.token::<u64>(),
    );

    let mut roads = vec![vec![0; n]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            roads[i][j] = c as i64 - '0' as i64;
        }
    }

    let convert = |q: usize, r: usize| -> usize { q * 5 + r };
    let m = 5 * n;
    let mut graph = vec![vec![0; m]; m];

    for i in 0..n {
        for j in 1..5 {
            let from = convert(i, j);
            let to = convert(i, j - 1);
            graph[from][to] = 1;
        }
    }

    for i in 0..n {
        for j in 0..n {
            let w = roads[i][j] as usize;

            if w == 0 {
                continue;
            }

            let from = convert(i, 0);
            let to = convert(j, w - 1);
            graph[from][to] = (graph[from][to] + 1) % MOD;
        }
    }

    let mat = matrix_pow(graph, t, m);

    writeln!(out, "{}", mat[convert(s, 0)][convert(e, 0)]).unwrap();
}
