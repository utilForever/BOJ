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

fn cross(stack: &Vec<(i64, i64)>, x: usize, y: usize) -> f64 {
    let t1 = stack[y].1 - stack[x].1;
    let t2 = stack[x].0 - stack[y].0;

    t1 as f64 / t2 as f64
}

fn insert(stack: &mut Vec<(i64, i64)>, size: &mut usize, x: i64, y: i64) {
    stack[*size] = (x, y);

    while *size > 1 && cross(stack, *size - 2, *size - 1) > cross(stack, *size - 1, *size) {
        stack[*size - 1] = stack[*size];
        *size -= 1;
    }

    *size += 1;
}

fn query(stack: &Vec<(i64, i64)>, size: &usize, last: &mut usize, x: i64) -> i64 {
    while *last + 1 < *size && cross(stack, *last, *last + 1) <= x as f64 {
        *last += 1;
    }

    x * stack[*last].0 as i64 + stack[*last].1 as i64
}

fn main() {
    let n = input_integers()[0] as usize;

    let nums = input_integers();
    let (a, b, c) = (nums[0], nums[1], nums[2]);

    let mut power = vec![0; n + 1];
    let mut accumulated_power = vec![0; n + 1];

    let nums = input_integers();

    for i in 1..=n {
        power[i] = nums[i - 1];
        accumulated_power[i] = accumulated_power[i - 1] + power[i];
    }

    let mut stack = vec![(0, 0); 1_000_001];
    let mut size = 0;

    insert(&mut stack, &mut size, 0, 0);

    let mut cost = vec![0; n + 1];
    let mut last = 0;

    for i in 1..=n {
        cost[i] = query(&stack, &size, &mut last, accumulated_power[i])
            + a * accumulated_power[i] * accumulated_power[i]
            + b * accumulated_power[i]
            + c;
        insert(
            &mut stack,
            &mut size,
            -2 * a * accumulated_power[i],
            a * accumulated_power[i] * accumulated_power[i] - b * accumulated_power[i] + cost[i],
        );
    }

    println!("{}", cost[n]);
}
