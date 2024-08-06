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
    let n = input_integers()[0];

    let mut num = 0;
    let mut cnt = 0;

    loop {
        num += 1;

        let mut temp = num;
        let mut flag = false;

        while temp > 0 {
            if temp % 1000 == 666 {
                flag = true;
                break;
            }

            temp /= 10;
        }

        if flag {
            cnt += 1;

            if n == cnt {
                break;
            }
        }
    }

    println!("{}", num);
}
