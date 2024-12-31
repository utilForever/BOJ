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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_dfs(graph: &Vec<Vec<usize>>, parent: &mut Vec<i64>, node: usize, par: i64) {
    parent[node] = par;

    for &child in graph[node].iter() {
        if child != par as usize {
            process_dfs(graph, parent, child, node as i64);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    if t == 1 {
        let n = scan.token::<usize>();
        let mut graph = vec![Vec::new(); n];

        for _ in 0..n - 1 {
            let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
            graph[u - 1].push(v - 1);
            graph[v - 1].push(u - 1);
        }

        let mut parent = vec![0; n];

        process_dfs(&graph, &mut parent, 0, -1);

        let sqrt = (n as f64).sqrt().ceil() as i64;
        let mut arr1 = vec![0; n];
        let mut arr2 = vec![0; n];
        let mut arr3 = vec![0; n];

        for i in 0..n {
            arr1[i] = parent[i] / sqrt;
            arr1[0] ^= arr1[i];
        }

        for i in 0..n {
            arr2[i] = parent[i] % sqrt;
            arr2[0] ^= arr2[i];
        }

        for i in 0..n {
            arr3[i] = parent[i];
            arr3[0] ^= arr3[i];
        }

        arr1[0] ^= 1;
        arr2[0] ^= 2;

        for val in arr1.iter() {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();

        for val in arr2.iter() {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();

        for val in arr3.iter() {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        let n = scan.token::<usize>();
        let sqrt = (n as f64).sqrt().ceil() as i64;

        let mut arr1 = vec![0; n];
        let mut arr2 = vec![0; n];

        for i in 0..n {
            arr1[i] = scan.token::<i64>();
        }

        for i in 0..n {
            arr2[i] = scan.token::<i64>();
        }

        for i in 1..n {
            arr1[0] ^= arr1[i];
            arr2[0] ^= arr2[i];
        }

        if arr1[0] > arr2[0] {
            std::mem::swap(&mut arr1, &mut arr2);
        }

        if arr1[0] == 0 {
            for i in 1..n {
                writeln!(out, "{} {}", arr1[i] + 1, i + 1).unwrap();
            }
        } else {
            for i in 1..n {
                let val = arr1[i] * sqrt + arr2[i];
                writeln!(out, "{} {}", val + 1, i + 1).unwrap();
            }
        }
    }
}
