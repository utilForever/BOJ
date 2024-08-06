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

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(parent: &mut Vec<usize>, mut a: usize, mut b: usize) {
    a = find(parent, a);
    b = find(parent, b);
    parent[a] = b;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut parent = vec![0; n + 1];
    for i in 1..=n {
        parent[i] = i;
    }

    let num_truth = scan.token::<usize>();
    let mut truth_people = Vec::new();

    for _ in 0..num_truth {
        truth_people.push(scan.token::<usize>());
    }

    let mut party = vec![Vec::new(); m];

    for i in 0..m {
        let num_people = scan.token::<usize>();
        let mut head = 0;

        for j in 0..num_people {
            let num = scan.token::<usize>();
            party[i].push(num);

            if j == 0 {
                head = num;
                continue;
            }

            process_union(&mut parent, head, num);
            head = num;
        }
    }

    let mut ans = 0;

    if num_truth == 0 {
        writeln!(out, "{}", m).unwrap();
    } else {
        for i in 0..m {
            let mut check = true;

            for j in 0..party[i].len() {
                let idx = party[i][j];

                for k in 0..truth_people.len() {
                    if find(&mut parent, idx) == find(&mut parent, truth_people[k]) {
                        check = false;
                        break;
                    }
                }

                if !check {
                    break;
                }
            }

            if check {
                ans += 1;
            }
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
