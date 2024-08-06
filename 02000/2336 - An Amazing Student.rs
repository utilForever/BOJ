use std::{cmp, io};

fn input_integers() -> Vec<usize> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<usize> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn init(tree: &mut Vec<usize>, node: usize, start: usize, end: usize) {
    tree[node] = 500_001;

    if start == end {
        return;
    }

    let mid = (start + end) / 2;
    init(tree, node * 2, start, mid);
    init(tree, node * 2 + 1, mid + 1, end);
}

fn get_minimum(
    tree: &Vec<usize>,
    node: usize,
    start: usize,
    end: usize,
    left: usize,
    right: usize,
) -> usize {
    if left > end || right < start {
        return 500_001;
    }

    if left <= start && right >= end {
        return tree[node];
    }

    let mid = (start + end) / 2;
    cmp::min(
        get_minimum(tree, node * 2, start, mid, left, right),
        get_minimum(tree, node * 2 + 1, mid + 1, end, left, right),
    )
}

fn update(tree: &mut Vec<usize>, node: usize, start: usize, end: usize, idx: usize, value: usize) {
    if idx < start || idx > end {
        return;
    }

    tree[node] = cmp::min(tree[node], value);

    if start == end {
        return;
    }

    let mid = (start + end) / 2;
    update(tree, node * 2, start, mid, idx, value);
    update(tree, node * 2 + 1, mid + 1, end, idx, value);
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut students = vec![(0, 0, 0); n + 1];

    let nums = input_integers();
    for i in 1..=n {
        students[nums[i - 1]].0 = i;
    }

    let nums = input_integers();
    for i in 1..=n {
        students[nums[i - 1]].1 = i;
    }

    let nums = input_integers();
    for i in 1..=n {
        students[nums[i - 1]].2 = i;
    }

    students.sort_by(|a, b| a.1.cmp(&b.1));

    let mut tree = vec![0; 500_001 * 4];
    init(&mut tree, 1, 1, n);

    let mut ans = 0;

    for i in 1..=n {
        let best = get_minimum(&tree, 1, 1, n, 1, students[i].0);
        if best > students[i].2 {
            ans += 1;
        }

        update(&mut tree, 1, 1, n, students[i].0, students[i].2);
    }

    println!("{}", ans);
}
