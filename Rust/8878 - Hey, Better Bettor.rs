use std::io;

fn input_floating_points() -> Vec<f64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<f64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let nums = input_floating_points();

    let mut refund = nums[0];
    let mut p = nums[1];

    refund /= 100.0;
    p /= 100.0;

    let mut win;
    let mut loss = 1;
    let mut max_win = 1;
    let mut max_expected_profit = 0.0;

    if p > 0.0 {
        loop {
            let mut prev_expected_profit = 0.0;
            let mut flag = false;

            win = max_win;

            loop {
                let factor = (1.0 - p) / p;
                let x = factor.powi(loss);
                let y = factor.powi(win + loss);

                let p_loss = (x - y) / (1.0 - y);
                let p_win = (1.0 - x) / (1.0 - y);

                let cur_expected_profit =
                    p_win * win as f64 - p_loss * loss as f64 * (1.0 - refund);

                if cur_expected_profit > max_expected_profit {
                    max_expected_profit = cur_expected_profit;
                    max_win = win;
                    flag = true;
                }

                if cur_expected_profit < prev_expected_profit {
                    break;
                }

                prev_expected_profit = cur_expected_profit;
                win += 1;
            }

            if flag {
                loss += 1;
            } else {
                break;
            }
        }
    }

    println!("{}", max_expected_profit);
}
