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

fn calculate_cases(
    graph: &Vec<Vec<i64>>,
    num_cases: &mut Vec<Vec<i64>>,
    cur_time: usize,
    end_time: usize,
    vertex: usize,
) -> i64 {
    if cur_time == end_time {
        if vertex == 7 {
            return 1;
        } else {
            return 0;
        }
    }

    if num_cases[vertex][cur_time] != -1 {
        return num_cases[vertex][cur_time];
    }

    num_cases[vertex][cur_time] = 0;

    for i in 0..graph[vertex].len() {
        let next_vertex = graph[vertex][i] as usize;
        num_cases[vertex][cur_time] +=
            calculate_cases(graph, num_cases, cur_time + 1, end_time, next_vertex);
        num_cases[vertex][cur_time] %= 1_000_000_007;
    }

    num_cases[vertex][cur_time] % 1_000_000_007
}

fn main() {
    let mut graph = vec![Vec::new(); 8];
    graph[0].push(1);
    graph[1].push(0);
    graph[0].push(2);
    graph[2].push(0);
    graph[2].push(3);
    graph[3].push(2);
    graph[1].push(3);
    graph[3].push(1);
    graph[1].push(4);
    graph[4].push(1);
    graph[3].push(4);
    graph[4].push(3);
    graph[3].push(5);
    graph[5].push(3);
    graph[4].push(5);
    graph[5].push(4);
    graph[4].push(6);
    graph[6].push(4);
    graph[5].push(6);
    graph[6].push(5);
    graph[5].push(7);
    graph[7].push(5);
    graph[6].push(7);
    graph[7].push(6);

    let d = input_integers()[0] as usize;
    let mut num_cases = vec![vec![-1; d]; 8];

    println!("{}", calculate_cases(&graph, &mut num_cases, 0, d, 7));
}
