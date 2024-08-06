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

fn get_gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        get_gcd(y, x % y)
    }
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut num = vec![String::new(); n];
    let mut arr = vec![0; n];
    let mut len = vec![0; n];

    for i in 0..n {
        io::stdin().read_line(&mut num[i]).unwrap();
        num[i] = num[i].trim().to_string();
        len[i] = num[i].chars().count();
    }

    let k = input_integers()[0] as usize;

    for i in 0..n {
        for j in 0..len[i] {
            arr[i] = arr[i] * 10 + (num[i].chars().nth(j).unwrap() as u8 - '0' as u8) as usize;
            arr[i] %= k;
        }
    }

    let mut ten = vec![0; 51];
    ten[0] = 1;

    for i in 1..=50 {
        ten[i] = ten[i - 1] * 10;
        ten[i] %= k;
    }

    let mut set = vec![vec![0; 100]; 1 << 15];
    set[0][0] = 1;

    for i in 0..(1 << n) {
        for j in 0..k {
            for l in 0..n {
                if i & (1 << l) == 0 {
                    let mut next = j * ten[len[l]];

                    next %= k;
                    next += arr[l];
                    next %= k;

                    set[i | (1 << l)][next] += set[i][j];
                }
            }
        }
    }

    let mut t1 = set[(1 << n) - 1][0];
    let mut t2 = 1;

    for i in 2..=n {
        t2 *= i;
    }

    let gcd = get_gcd(t1, t2);
    t1 /= gcd;
    t2 /= gcd;

    println!("{}/{}", t1, t2);
}
