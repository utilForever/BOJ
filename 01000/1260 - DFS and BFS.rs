use io::Write;
use std::{
    collections::VecDeque,
    io::{self, BufWriter, StdoutLock},
};

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
    out: &mut BufWriter<StdoutLock>,
    graph: &Vec<Vec<usize>>,
    num_vertices: usize,
    start: usize,
) {
    let mut stack = Vec::new();
    let mut check = vec![false; num_vertices + 1];

    stack.push(start);
    check[start] = true;

    write!(out, "{} ", start).unwrap();

    while !stack.is_empty() {
        let cur_node = stack.last().unwrap();
        let mut can_connect = false;

        for vertex in graph[*cur_node].iter() {
            if !check[*vertex] {
                stack.push(*vertex);
                check[*vertex] = true;

                write!(out, "{} ", vertex).unwrap();

                can_connect = true;
                break;
            }
        }

        if !can_connect {
            stack.pop();
        }
    }
}

fn process_bfs(
    out: &mut BufWriter<StdoutLock>,
    graph: &Vec<Vec<usize>>,
    num_vertices: usize,
    start: usize,
) {
    let mut queue = VecDeque::new();
    let mut check = vec![false; num_vertices + 1];

    queue.push_back(start);
    check[start] = true;

    write!(out, "{} ", start).unwrap();

    while !queue.is_empty() {
        let cur_node = queue.front().unwrap();

        for vertex in graph[*cur_node].iter() {
            if !check[*vertex] {
                queue.push_back(*vertex);
                check[*vertex] = true;

                write!(out, "{} ", vertex).unwrap();
            }
        }

        queue.pop_front();
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let nums = input_integers();
    let (n, m, v) = (nums[0] as usize, nums[1] as usize, nums[2] as usize);

    let mut graph = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let nums = input_integers();
        let (a, b) = (nums[0] as usize, nums[1] as usize);

        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 1..=n {
        graph[i].sort();
    }

    process_dfs(&mut out, &graph, n, v);

    writeln!(out).unwrap();

    process_bfs(&mut out, &graph, n, v);
}
