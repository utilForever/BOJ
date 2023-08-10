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

fn get_connected_component(graph: &Vec<Vec<usize>>, num_vertices: usize) -> usize {
    let mut stack = Vec::new();
    let mut check = vec![false; num_vertices + 1];
    let mut cnt = 0;

    for i in 0..num_vertices {
        if check[i] {
            continue;
        }

        cnt += 1;

        stack.push(i);
        check[i] = true;

        while !stack.is_empty() {
            let cur_node = *stack.last().unwrap();
            let mut can_connect = false;

            for vertex in graph[cur_node].iter() {
                if !check[*vertex] {
                    stack.push(*vertex);
                    check[*vertex] = true;
                    can_connect = true;
                    break;
                }
            }

            if !can_connect {
                stack.pop();
            }
        }
    }

    cnt
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut graph = vec![Vec::new(); n];

        let k = scan.token::<i64>();

        for _ in 0..k {
            let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
            graph[a].push(b);
            graph[b].push(a);
        }

        writeln!(
            out,
            "{}",
            if get_connected_component(&graph, n) == 1 {
                "Connected."
            } else {
                "Not connected."
            }
        )
        .unwrap();
    }
}
