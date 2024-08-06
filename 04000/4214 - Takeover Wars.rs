use std::io;

fn input_integers() -> Vec<usize> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<usize> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

#[derive(PartialEq)]
enum Corporation {
    Takeover,
    Buyout,
}

fn process_greedy(
    subsidiaries_takeover: &Vec<usize>,
    subsidiaries_buyout: &Vec<usize>,
    num_subsidiaries_takeover: usize,
    num_subsidiaries_buyout: usize,
    max_value_takeover: usize,
    max_value_buyout: usize,
    turn: Corporation,
) -> Corporation {
    match turn {
        Corporation::Takeover => {
            if num_subsidiaries_buyout == 0 {
                return Corporation::Takeover;
            } else if num_subsidiaries_takeover == 1 && max_value_takeover <= max_value_buyout {
                return Corporation::Buyout;
            }

            if max_value_takeover + subsidiaries_takeover[num_subsidiaries_takeover - 1]
                > max_value_buyout
            {
                let ret = process_greedy(
                    subsidiaries_takeover,
                    subsidiaries_buyout,
                    num_subsidiaries_takeover - 1,
                    num_subsidiaries_buyout,
                    max_value_takeover + subsidiaries_takeover[num_subsidiaries_takeover - 1],
                    max_value_buyout,
                    Corporation::Buyout,
                );

                if ret == Corporation::Takeover {
                    return Corporation::Takeover;
                }
            }

            if max_value_takeover > max_value_buyout {
                let ret = process_greedy(
                    subsidiaries_takeover,
                    subsidiaries_buyout,
                    num_subsidiaries_takeover,
                    num_subsidiaries_buyout - 1,
                    max_value_takeover,
                    subsidiaries_buyout[num_subsidiaries_buyout - 1],
                    Corporation::Buyout,
                );

                if ret == Corporation::Takeover {
                    return Corporation::Takeover;
                }
            }

            Corporation::Buyout
        }
        Corporation::Buyout => {
            if num_subsidiaries_takeover == 0 {
                return Corporation::Buyout;
            } else if num_subsidiaries_buyout == 1 && max_value_buyout <= max_value_takeover {
                return Corporation::Takeover;
            }

            if max_value_buyout + subsidiaries_buyout[num_subsidiaries_buyout - 1]
                > max_value_takeover
            {
                let ret = process_greedy(
                    subsidiaries_takeover,
                    subsidiaries_buyout,
                    num_subsidiaries_takeover,
                    num_subsidiaries_buyout - 1,
                    max_value_takeover,
                    max_value_buyout + subsidiaries_buyout[num_subsidiaries_buyout - 1],
                    Corporation::Takeover,
                );

                if ret == Corporation::Buyout {
                    return Corporation::Buyout;
                }
            }

            if max_value_buyout > max_value_takeover {
                let ret = process_greedy(
                    subsidiaries_takeover,
                    subsidiaries_buyout,
                    num_subsidiaries_takeover - 1,
                    num_subsidiaries_buyout,
                    subsidiaries_takeover[num_subsidiaries_takeover - 1],
                    max_value_buyout,
                    Corporation::Takeover,
                );

                if ret == Corporation::Buyout {
                    return Corporation::Buyout;
                }
            }

            Corporation::Takeover
        }
    }
}

fn main() {
    let mut num_cases = 1;

    loop {
        let nums = input_integers();
        if nums.len() == 0 {
            break;
        }

        let (n, m) = (nums[0], nums[1]);
        let mut subsidiaries_takeover = vec![0; n + 1];
        let mut subsidiaries_buyout = vec![0; m + 1];

        let nums = input_integers();
        for i in 1..=n {
            subsidiaries_takeover[i] = nums[i - 1];
        }

        let nums = input_integers();
        for i in 1..=m {
            subsidiaries_buyout[i] = nums[i - 1];
        }

        subsidiaries_takeover.sort();
        subsidiaries_buyout.sort();

        let ret = process_greedy(
            &subsidiaries_takeover,
            &subsidiaries_buyout,
            n,
            m,
            subsidiaries_takeover[n],
            subsidiaries_buyout[m],
            Corporation::Takeover,
        );

        println!(
            "Case {}: {}",
            num_cases,
            match ret {
                Corporation::Takeover => "Takeover Incorporated",
                Corporation::Buyout => "Buyout Limited",
            }
        );

        num_cases += 1;
    }
}
