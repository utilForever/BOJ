use std::io;

static UNIT: i64 = 50;

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
    let nums = input_integers();
    let (k, r) = (nums[0], nums[1]);

    let mut count = 0;
    let mut i = 0;

    while i < r + UNIT {
        let mut j = 0;

        while j < r + UNIT {
            if (i.pow(2) + j.pow(2)) * k.pow(2) > r.pow(2)
                || ((i + UNIT).pow(2) + (j + UNIT).pow(2)) * k.pow(2) < r.pow(2)
            {
                j += UNIT;
                continue;
            }

            for x in i..(i + UNIT) {
                for y in j..(j + UNIT) {
                    if (x.pow(2) + y.pow(2)) * k.pow(2) < r.pow(2)
                        && r.pow(2) < ((x + 1).pow(2) + (y + 1).pow(2)) * k.pow(2)
                    {
                        count += 1;
                    }
                }
            }

            j += UNIT;
        }

        i += UNIT;
    }

    println!("{}", 4 * count);
}
