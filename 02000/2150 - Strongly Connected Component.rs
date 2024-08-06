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

fn process_dfs(
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    stack: &mut Vec<usize>,
    top: &mut usize,
    node: usize,
) {
    visited[node] = true;

    for next in graph[node].iter() {
        if !visited[*next] {
            process_dfs(graph, visited, stack, top, *next);
        }
    }

    stack[*top] = node;
    *top += 1;
}

fn process_dfs_rev(
    scc_group: &mut Vec<Vec<usize>>,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    node: usize,
) {
    visited[node] = true;

    let len = scc_group.len();
    scc_group[len - 1].push(node);

    for next in graph[node].iter() {
        if !visited[*next] {
            process_dfs_rev(scc_group, graph, visited, *next);
        }
    }
}

fn main() {
    let nums = input_integers();
    let (v, e) = (nums[0] as usize, nums[1] as usize);

    let mut graph = vec![Vec::new(); 10001];
    let mut rev_graph = vec![Vec::new(); 10001];

    for _ in 0..e {
        let nums = input_integers();
        let (a, b) = (nums[0] as usize, nums[1] as usize);

        graph[a].push(b);
        rev_graph[b].push(a);
    }

    let mut scc_group = Vec::new();
    let mut visited = vec![false; 10001];
    let mut stack = vec![0; 10001];
    let mut top = 0;

    for idx in 1..=v {
        if !visited[idx] {
            process_dfs(&graph, &mut visited, &mut stack, &mut top, idx);
        }
    }

    visited.fill(false);

    while top > 0 {
        let node = stack[top - 1];
        top -= 1;

        if !visited[node] {
            scc_group.push(Vec::new());
            process_dfs_rev(&mut scc_group, &rev_graph, &mut visited, node);
        }
    }

    for group in scc_group.iter_mut() {
        group.sort();
    }

    scc_group.sort();

    println!("{}", scc_group.len());

    for group in scc_group.iter() {
        for node in group.iter() {
            print!("{} ", node);
        }

        println!("-1");
    }
}
