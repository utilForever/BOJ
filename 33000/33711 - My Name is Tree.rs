use io::Write;
use std::{
    collections::{HashMap, VecDeque},
    io, str,
};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let w = scan.token::<i64>();
    let mut names: HashMap<String, Vec<usize>> = HashMap::new();

    for _ in 0..w {
        let (a, b) = (scan.token::<usize>() - 1, scan.token::<String>());
        names.entry(b).or_default().push(a);
    }

    let mut graph = vec![Vec::new(); n];

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited_group = vec![0; n];
    let mut visited_data = vec![(0, 0); n];
    let mut queue = VecDeque::new();
    let mut group_idx = 0;

    for (_, persons) in names.iter() {
        if persons.len() <= 1 {
            continue;
        }

        queue.clear();
        group_idx += 1;

        for &source in persons.iter() {
            if visited_group[source] != group_idx {
                visited_group[source] = group_idx;
                visited_data[source] = (source, 0);
                queue.push_back(source);    
            }
        }

        while let Some(curr) = queue.pop_front() {
            let (person1, dist1) = visited_data[curr];

            if dist1 == k {
                continue;
            }

            for &next in graph[curr].iter() {
                if visited_group[next] == group_idx {
                    let (person2, dist2) = visited_data[next];

                    if person1 != person2 && dist1 + dist2 + 1 <= k {
                        writeln!(out, "POWERFUL CODING JungHwan").unwrap();
                        return;
                    }
                } else {
                    visited_group[next] = group_idx;
                    visited_data[next] = (person1, dist1 + 1);
                    queue.push_back(next);
                }
            }
        }
    }

    writeln!(out, "so sad").unwrap();
}
