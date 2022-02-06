use std::{cmp, io};

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

fn convert_idx(val: i64, n: i64) -> usize {
    if val > n {
        (val - n) as usize
    } else {
        (val + n) as usize
    }
}

fn process_scc(
    scc_group: &mut Vec<Vec<usize>>,
    scc: &mut Vec<i64>,
    graph: &Vec<Vec<i64>>,
    visited: &mut Vec<i64>,
    stack: &mut Vec<i64>,
    top: &mut i64,
    cnt: &mut i64,
    node: usize,
) -> i64 {
    *cnt += 1;
    visited[node] = *cnt;

    stack[*top as usize] = node as i64;
    *top += 1;

    let mut ret = visited[node];

    for next in graph[node].iter() {
        if visited[*next as usize] == 0 {
            ret = cmp::min(
                ret,
                process_scc(
                    scc_group,
                    scc,
                    graph,
                    visited,
                    stack,
                    top,
                    cnt,
                    *next as usize,
                ),
            );
        } else if scc[*next as usize] == 0 {
            ret = cmp::min(ret, visited[*next as usize]);
        }
    }

    if ret == visited[node] {
        scc_group.push(Vec::new());

        loop {
            let now = stack[(*top - 1) as usize];
            *top -= 1;

            scc[now as usize] = scc_group.len() as i64;
            scc_group[(scc[now as usize] - 1) as usize].push(now as usize);

            if now == node as i64 {
                break;
            }
        }
    }

    ret
}

fn main() {
    let nums = input_integers();
    let (n, m) = (nums[0] as usize, nums[1] as usize);

    let mut graph = vec![Vec::new(); 20002];

    for _ in 0..m {
        let nums = input_integers();
        let (mut a, mut b) = (nums[0], nums[1]);

        if a < 0 {
            a = -a + n as i64;
        }
        if b < 0 {
            b = -b + n as i64;
        }

        graph[convert_idx(a, n as i64)].push(b);
        graph[convert_idx(b, n as i64)].push(a);
    }

    let mut scc_group = Vec::new();
    let mut scc = vec![0; 20002];
    let mut visited = vec![0; 20002];
    let mut stack = vec![0; 20002];
    let mut top = 0;
    let mut cnt = 0;

    for i in 1..=(2 * n) {
        if visited[i] == 0 {
            process_scc(
                &mut scc_group,
                &mut scc,
                &graph,
                &mut visited,
                &mut stack,
                &mut top,
                &mut cnt,
                i,
            );
        }
    }

    for i in 1..=n {
        if scc[i] == scc[convert_idx(i as i64, n as i64)] {
            print!("0");
            return;
        }
    }

    println!("1");
    for i in 1..=n {
        print!(
            "{} ",
            if scc[i] < scc[convert_idx(i as i64, n as i64)] {
                "1"
            } else {
                "0"
            }
        );
    }
}
