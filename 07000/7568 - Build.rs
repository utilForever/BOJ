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

fn main() {
    let n = input_integers()[0] as usize;
    let mut people = vec![(0, 0, 0); n];

    for i in 0..n {
        let nums = input_integers();
        people[i] = (nums[0], nums[1], 1);
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if people[i].0 > people[j].0 && people[i].1 > people[j].1 {
                people[j].2 += 1;
            }
        }
    }

    for i in 0..n {
        print!("{} ", people[i].2);
    }
    println!();
}
