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
    let mut stones = input_integers();

    let mut num_one_stone = 0;

    for stone in stones.iter() {
        if stone == &1 {
            num_one_stone += 1;
        }
    }

    if num_one_stone == n as i64 {
        if n % 2 != 0 {
            println!("cubelover");
        } else {
            println!("koosaga");
        }
    } else {
        if num_one_stone > 0 && num_one_stone % 2 == 0 {
            for stone in stones.iter_mut() {
                if stone != &1 {
                    *stone = 1;
                    break;
                }
            }
        }

        let mut ret = 0;

        for stone in stones.iter() {
            ret ^= stone;
        }
    
        if ret == 0 {
            println!("cubelover");
        } else {
            println!("koosaga");
        }
    }
}
