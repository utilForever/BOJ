use std::{cmp, io};

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn get_maximum(tree: &mut Vec<i32>, node: i32, start: i32, end: i32, left: i32, right: i32) -> i32 {
    if left > end || right < start {
        return 0;
    }

    if left <= start && right >= end {
        return tree[node as usize];
    }

    cmp::max(
        get_maximum(tree, 2 * node, start, (start + end) / 2, left, right),
        get_maximum(tree, 2 * node + 1, (start + end) / 2 + 1, end, left, right),
    )
}

fn update(tree: &mut Vec<i32>, node: i32, start: i32, end: i32, i: i32, value: i32) {
    if i < start || i > end {
        return;
    }

    tree[node as usize] = cmp::max(tree[node as usize], value);

    if start != end {
        update(tree, 2 * node, start, (start + end) / 2, i, value);
        update(tree, 2 * node + 1, (start + end) / 2 + 1, end, i, value);
    }
}

fn main() {
    let n = input_integers()[0] as usize;

    let sequence = input_integers();
    let mut tree = vec![0; 4 * 1_000_000];

    let mut ans = 0;

    for i in 0..n {
        let a = sequence[i];
        let cur = get_maximum(&mut tree, 1, 1, 1_000_000, 1, a - 1);

        if ans < cur + 1 {
            ans = cur + 1;
        }

        update(&mut tree, 1, 1, 1_000_000, a, cur + 1);
    }

    println!("{}", ans);
}
