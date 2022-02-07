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

fn update(tree: &mut Vec<i64>, x: i64, diff: i64) {
    let mut idx = x;

    while idx < tree.len() as i64 {
        tree[idx as usize] += diff;
        idx += idx & (-idx);
    }
}

fn sum(tree: &Vec<i64>, x: i64) -> i64 {
    let mut ans = 0;
    let mut idx = x;

    while idx > 0 {
        ans += tree[idx as usize];
        idx -= idx & (-idx);
    }

    ans
}

fn main() {
    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let nums = input_integers();
        let (n, m) = (nums[0] as usize, nums[1] as usize);

        let mut tree = vec![0; n + m + 1];
        let mut position = vec![0; n + 1];

        for i in 1..=n {
            update(&mut tree, (m + i) as i64, 1);
            position[i] = (m + i) as i64;
        }

        let nums = input_integers();
        let mut idx = 0;

        for i in (1..=m).rev() {
            let num = nums[idx] as usize;

            print!("{} ", sum(&tree, position[num] - 1));

            update(&mut tree, position[num], -1);
            position[num] = i as i64;
            update(&mut tree, position[num], 1);

            idx += 1;
        }

        println!("");
    }
}
