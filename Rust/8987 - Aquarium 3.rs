use io::Write;
use std::{cmp, collections::BinaryHeap, io, str};

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

fn init(
    arr: &Vec<(usize, usize)>,
    tree: &mut Vec<(usize, usize)>,
    node: usize,
    start: usize,
    end: usize,
) {
    if start == end {
        tree[node] = (arr[start].1, start as usize);
    } else {
        init(arr, tree, node * 2, start, (start + end) / 2);
        init(arr, tree, node * 2 + 1, (start + end) / 2 + 1, end);

        if tree[node * 2].0 <= tree[node * 2 + 1].0 {
            tree[node] = tree[node * 2];
        } else {
            tree[node] = tree[node * 2 + 1];
        }
    }
}

fn query(
    arr: &Vec<(usize, usize)>,
    tree: &Vec<(usize, usize)>,
    node: usize,
    start: usize,
    end: usize,
    i: usize,
    j: usize,
) -> (usize, usize) {
    if i > end || j < start {
        return (1_000_000_007, 0);
    }

    if i <= start && j >= end {
        return tree[node] as (usize, usize);
    }

    let left = query(arr, tree, node * 2, start, (start + end) / 2, i, j);
    let right = query(arr, tree, node * 2 + 1, (start + end) / 2 + 1, end, i, j);

    if left.0 <= right.0 {
        left
    } else {
        right
    }
}

fn get_area(
    arr: &Vec<(usize, usize)>,
    tree: &Vec<(usize, usize)>,
    priority_queue: &mut BinaryHeap<usize>,
    n: &usize,
    start: usize,
    end: usize,
    accumulated_height: usize,
) -> usize {
    if start > end {
        return 0;
    }

    let (height, idx) = query(arr, tree, 1, 1, n / 2 - 1, start, end);
    let cur_area = (arr[end + 1].0 - arr[start].0) * (height - accumulated_height);

    let (left_area, right_area) = (
        get_area(
            arr,
            tree,
            priority_queue,
            n,
            start,
            idx as usize - 1,
            height,
        ),
        get_area(arr, tree, priority_queue, n, idx as usize + 1, end, height),
    );
    priority_queue.push(cmp::min(left_area, right_area));

    cur_area + cmp::max(left_area, right_area)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut coords = vec![(0, 0); n / 2 + 1];

    for i in 1..=(n / 2) {
        (coords[i].0, coords[i].1) = (scan.token::<usize>(), scan.token::<usize>());
        (coords[i].0, coords[i].1) = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut tree = vec![(0, 0); 2 * n];

    init(&coords, &mut tree, 1, 1, n / 2 - 1);

    let mut k = scan.token::<usize>();
    let mut priority_queue = BinaryHeap::new();

    let area = get_area(&coords, &tree, &mut priority_queue, &n, 1, n / 2 - 1, 0);
    priority_queue.push(area);

    let mut ret = 0;

    while !priority_queue.is_empty() {
        ret += priority_queue.pop().unwrap();
        k -= 1;

        if k == 0 {
            break;
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
