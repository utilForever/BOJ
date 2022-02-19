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

#[inline]
fn calculate_risk(accumulated_jailbreak_power: &Vec<usize>, i: usize, j: usize) -> i64 {
    if i > j {
        0
    } else {
        ((accumulated_jailbreak_power[j] - accumulated_jailbreak_power[i - 1]) * (j - i + 1)) as i64
    }
}

fn calculate_minimum_risk(
    jailbreak_risk: &mut Vec<Vec<i64>>,
    assigned_guard_idx: &mut Vec<Vec<i64>>,
    accumulated_jailbreak_power: &Vec<usize>,
    idx: usize,
    left: i64,
    right: i64,
    p_left: usize,
    p_right: usize,
) {
    if left > right {
        return;
    }

    let mid = ((left + right) / 2) as usize;

    jailbreak_risk[idx][mid] = -1;
    assigned_guard_idx[idx][mid] = -1;

    for i in p_left..=p_right {
        let risk =
            jailbreak_risk[idx - 1][i] + calculate_risk(accumulated_jailbreak_power, i + 1, mid);

        if jailbreak_risk[idx][mid] == -1 || jailbreak_risk[idx][mid] > risk {
            jailbreak_risk[idx][mid] = risk;
            assigned_guard_idx[idx][mid] = i as i64;
        }
    }

    calculate_minimum_risk(
        jailbreak_risk,
        assigned_guard_idx,
        accumulated_jailbreak_power,
        idx,
        left,
        mid as i64 - 1,
        p_left,
        assigned_guard_idx[idx][mid] as usize,
    );
    calculate_minimum_risk(
        jailbreak_risk,
        assigned_guard_idx,
        accumulated_jailbreak_power,
        idx,
        mid as i64 + 1,
        right,
        assigned_guard_idx[idx][mid] as usize,
        p_right,
    );
}

fn main() {
    let nums = input_integers();
    let (l, g) = (nums[0], nums[1]);

    let mut jailbreak_power = input_integers();
    jailbreak_power.insert(0, 0);

    let accumulated_jailbreak_power: Vec<usize> = jailbreak_power
        .iter()
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect();

    let mut jailbreak_risk = vec![vec![0; l + 1]; g + 1];
    let mut assigned_guard_idx = vec![vec![0; l + 1]; g + 1];

    for i in 1..=l {
        jailbreak_risk[1][i] = calculate_risk(&accumulated_jailbreak_power, 1, i);
        assigned_guard_idx[1][i] = 0;
    }

    for i in 2..=g {
        calculate_minimum_risk(
            &mut jailbreak_risk,
            &mut assigned_guard_idx,
            &accumulated_jailbreak_power,
            i,
            0,
            l as i64,
            0,
            l,
        );
    }

    println!("{}", jailbreak_risk[g][l]);
}
