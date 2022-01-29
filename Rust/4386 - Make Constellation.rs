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

fn input_floating_points() -> Vec<f64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<f64> = s
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
    let n = input_integers()[0] as usize;
    let mut vertices: Vec<(usize, usize)> = vec![(0, 0); n];
    let mut edges: Vec<(usize, usize, f64)> = Vec::new();

    for i in 1..=n {
        let nums = input_floating_points();
        let (x, y) = (nums[0], nums[1]);
        vertices[i - 1] = (x as usize, y as usize);
    }

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let dist = (((vertices[i].0 - vertices[j].0).pow(2)
                + (vertices[i].1 - vertices[j].1).pow(2)) as f64)
                .sqrt();
            edges.push((i + 1, j + 1, dist));
        }
    }

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut root = vec![0_usize; n + 1];
    let mut visited = vec![false; n + 1];
    let mut sum = 0.0;

    for i in 0..edges.len() {
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
