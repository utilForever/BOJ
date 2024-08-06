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

fn process_dfs(
    tunnels: &Vec<Vec<usize>>,
    path_parent: &mut Vec<usize>,
    path_kid1: &mut Vec<usize>,
    path_kid2: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    num_path_kid1: &mut usize,
    num_path_kid2: &mut usize,
    idx: usize,
) {
    if visited[idx] || num_path_kid1 == num_path_kid2 {
        return;
    }

    path_kid1[idx] = idx;
    *num_path_kid1 -= 1;
    path_parent.push(idx);

    visited[idx] = true;

    for chamber in tunnels[idx].iter() {
        if visited[*chamber] {
            continue;
        }

        process_dfs(
            tunnels,
            path_parent,
            path_kid1,
            path_kid2,
            visited,
            num_path_kid1,
            num_path_kid2,
            *chamber,
        );
    }

    if num_path_kid1 == num_path_kid2 {
        return;
    }

    path_kid2[idx] = idx;
    *num_path_kid2 += 1;
    path_parent.pop();
}

// Reference: https://2020.nwerc.eu/files/nwerc2020slides.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (c, t) = (scan.token::<usize>(), scan.token::<usize>());
    let mut tunnels = vec![Vec::new(); c + 1];

    for _ in 0..t {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        tunnels[a].push(b);
        tunnels[b].push(a);
    }

    let mut path_parent = Vec::new();
    let mut path_kid1 = vec![0; c + 1];
    let mut path_kid2 = vec![0; c + 1];
    let mut visited = vec![false; c + 1];
    let mut num_path_kid1 = c;
    let mut num_path_kid2 = 0;

    process_dfs(
        &tunnels,
        &mut path_parent,
        &mut path_kid1,
        &mut path_kid2,
        &mut visited,
        &mut num_path_kid1,
        &mut num_path_kid2,
        1,
    );

    writeln!(out, "{} {}", path_parent.len(), num_path_kid1).unwrap();

    for path in path_parent.iter() {
        write!(out, "{} ", path).unwrap();
    }
    writeln!(out).unwrap();

    if num_path_kid1 == 0 {
        return;
    }

    for i in 1..=c {
        if path_kid1[i] != 0 {
            continue;
        }

        write!(out, "{} ", i).unwrap();
    }
    writeln!(out).unwrap();

    for path in path_kid2.iter() {
        if *path == 0 {
            continue;
        }

        write!(out, "{} ", path).unwrap();
    }
    writeln!(out).unwrap();
}
