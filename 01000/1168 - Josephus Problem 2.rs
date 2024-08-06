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

fn init(tree: &mut Vec<i64>, node: usize, start: usize, end: usize) {
    if start == end {
        tree[node] = 1;
        return;
    }

    let mid = (start + end) / 2;

    init(tree, node * 2, start, mid);
    init(tree, node * 2 + 1, mid + 1, end);

    tree[node] = tree[node * 2] + tree[node * 2 + 1];
}

fn update(tree: &mut Vec<i64>, node: usize, start: usize, end: usize, remove: usize) {
    tree[node] -= 1;

    if start == end {
        return;
    }

    let mid = (start + end) / 2;

    if remove <= mid {
        update(tree, node * 2, start, mid, remove);
    } else {
        update(tree, node * 2 + 1, mid + 1, end, remove);
    }
}

fn query(tree: &mut Vec<i64>, node: usize, start: usize, end: usize, order: i64) -> i64 {
    if start == end {
        return start as i64;
    }

    let mid = (start + end) / 2;

    if order <= tree[node * 2] {
        return query(tree, node * 2, start, mid, order);
    } else {
        return query(tree, node * 2 + 1, mid + 1, end, order - tree[node * 2]);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut tree = vec![0; 4 * (n + 1)];
    let mut index = 1;

    init(&mut tree, 1, 1, n);

    write!(out, "<").unwrap();

    for i in 0..n {
        let num_people = n - i;
        index += k - 1;

        if index % num_people == 0 {
            index = num_people;
        } else if index > num_people {
            index %= num_people;
        }

        let ret = query(&mut tree, 1, 1, n, index as i64);
        update(&mut tree, 1, 1, n, ret as usize);

        write!(out, "{}", ret).unwrap();

        if i < n - 1 {
            write!(out, ", ").unwrap();
        }
    }

    writeln!(out, ">").unwrap();
}
