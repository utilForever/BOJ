use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn find_root(root: &mut Vec<usize>, idx: usize) -> usize {
    if root[idx] == 0 {
        root[idx] = idx;
        return idx;
    }

    if root[idx] == idx {
        return idx;
    }

    root[idx] = find_root(root, root[idx]);
    root[idx]
}

fn connect_vertices(root: &mut Vec<usize>, a: usize, b: usize) {
    let a = find_root(root, a);
    let b = find_root(root, b);

    if a != b {
        root[a] = b;
    }
}

fn main() {
    let nums = input_integers();
    let (v, e) = (nums[0] as usize, nums[1] as usize);

    let mut edges = Vec::new();

    for _ in 0..e {
        let nums = input_integers();
        let (a, b, c) = (nums[0] as usize, nums[1] as usize, nums[2]);
        edges.push((a, b, c));
    }

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut root = vec![0_usize; v + 1];
    let mut visited = vec![false; v + 1];
    let mut sum = 0;

    for i in 0..e {
        if find_root(&mut root, edges[i].0) == find_root(&mut root, edges[i].1) {
            continue;
        }

        visited[edges[i].0] = true;
        visited[edges[i].1] = true;

        connect_vertices(&mut root, edges[i].0, edges[i].1);

        sum += edges[i].2;
    }

    println!("{}", sum);
}
