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
    let mut palaces = vec![(0, 0); n];

    for i in 0..n {
        let nums = input_integers();
        palaces[i] = (nums[0], nums[1]);
    }

    let mut ret = 0;
    for palace in palaces.iter() {
        ret ^= ((palace.0 / 3) ^ (palace.1 / 3)) * 3 + (palace.0 + palace.1) % 3;
    }

    if ret == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
