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

fn process_dfs(
    person: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    assigned_work: &mut Vec<usize>,
    n: usize,
) -> bool {
    for i in 0..person[n].len() {
        let work = person[n][i];

        if check[work] {
            continue;
        }

        check[work] = true;

        if assigned_work[work] == 0
            || process_dfs(person, check, assigned_work, assigned_work[work])
        {
            assigned_work[work] = n;
            return true;
        }
    }

    false
}

fn main() {
    let nums = input_integers();
    let (n, _, k) = (nums[0] as usize, nums[1] as usize, nums[2] as usize);

    let mut person = vec![Vec::new(); 1001];

    for i in 1..=n {
        let nums = input_integers();
        let num_works = nums[0] as usize;

        for j in 0..num_works {
            person[i].push(nums[j + 1] as usize);
        }
    }

    let mut check = vec![false; 1001];
    let mut assigned_work = vec![0; 1001];
    let mut max_work = 0;

    for i in 1..=n {
        check.fill(false);

        if process_dfs(&person, &mut check, &mut assigned_work, i) {
            max_work += 1;
        }
    }

    let mut exceed_count = 0;

    loop {
        let mut is_success = false;

        for i in 1..=n {
            if exceed_count == k {
                break;
            }
            
            check.fill(false);

            if process_dfs(&person, &mut check, &mut assigned_work, i) {
                is_success = true;
                exceed_count += 1;
                max_work += 1;
                break;
            }
        }

        if !is_success {
            break;
        }
    }

    println!("{}", max_work);
}
