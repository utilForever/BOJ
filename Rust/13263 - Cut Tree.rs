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

fn cross(stack: &Vec<(i64, i64)>, x: i64, y: i64) -> f64 {
    let t1 = (stack[y as usize].1 - stack[x as usize].1) as f64;
    let t2 = (stack[x as usize].0 - stack[y as usize].0) as f64;

    t1 / t2
}

fn insert(stack: &mut Vec<(i64, i64)>, size: &mut i64, x: i64, y: i64) {
    stack[*size as usize] = (x, y);

    while *size > 1 && cross(stack, *size - 2, *size - 1) > cross(stack, *size - 1, *size) {
        stack.swap(*size as usize - 1, *size as usize);
        *size -= 1;
    }

    *size += 1;
}

fn query(stack: &Vec<(i64, i64)>, size: i64, last: &mut i64, x: i64) -> i64 {
    while *last + 1 < size && cross(stack, *last, *last + 1) <= x as f64 {
        *last += 1;
    }

    x * stack[*last as usize].0 + stack[*last as usize].1
}

fn main() {
    let n = input_integers()[0] as usize;
    let a = input_integers();
    let b = input_integers();

    let mut stack = vec![(0, 0); n + 1];
    let mut cost = vec![0; n];
    let (mut size, mut last) = (0, 0);

    insert(&mut stack, &mut size, b[0], 0);

    for i in 1..n {
        cost[i] = query(&stack, size, &mut last, a[i]);
        insert(&mut stack, &mut size, b[i], cost[i]);
    }

    println!("{}", cost[n - 1]);
}
