use std::{collections::HashMap, io};

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
    let m = input_integers()[0] as usize;
    let _ = input_integers()[0] as usize;
    let moves = input_integers();

    let mut mask = (1 << 22) - 1;
    let mut ans = -1;

    let mut last = HashMap::new();
    let mut r = Vec::new();

    let mut idx = 0;

    loop {
        if idx > m {
            break;
        }

        mask <<= 1;
        ans += 1;

        for j in moves.iter() {
            if (mask & (1 << j)) == 0 {
                mask += 1;
                ans -= 1;
                break;
            }
        }

        mask &= (1 << 22) - 1;

        if last.contains_key(&mask) {
            let length = (idx as i64 - last[&mask]) as usize;
            let cnt = (m - idx) / length;

            idx += cnt * length;
            ans += cnt as i64 * (ans - r[last[&mask] as usize]) as i64;
        }

        last.insert(mask, idx as i64);
        r.push(ans);

        idx += 1;
    }

    println!("{}", ans);
}
