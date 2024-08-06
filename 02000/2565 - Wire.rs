use std::io;

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

fn main() {
    let n = input_integers()[0] as usize;
    let mut wires = vec![(0, 0); n];
    let mut longest = vec![0; n];

    for i in 0..n {
        let nums = input_integers();
        wires[i] = (nums[0], nums[1]);
    }

    wires.sort();

    for i in 0..n {
        longest[i] = 1;

        for j in 0..i {
            if wires[i].0 > wires[j].0 && wires[i].1 > wires[j].1 && longest[i] < longest[j] + 1 {
                longest[i] = longest[j] + 1;
            }
        }
    }

    let ans = longest.iter().max().unwrap().clone();
    println!("{}", n - ans);
}
