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

fn main() {
    let nums = input_integers();
    let (n, mut m) = (nums[0], nums[1]);

    let mut crop_pos = input_integers();
    crop_pos.insert(0, 0);
    crop_pos.push(n);

    crop_pos.sort();
    m += 2;

    let mut minimum_pos = vec![vec![0; m + 1]; m + 1];
    let mut minimum_cost = vec![vec![0; m + 1]; m + 1];

    for i in 0..m {
        minimum_pos[i][i] = i;
        minimum_cost[i][i] = 0;

        for j in (i + 1)..m {
            minimum_pos[i][j] = i;
            minimum_cost[i][j] = 0;
        }
    }

    for i in 2..=m {
        let mut j = 0;

        while i + j < m {
            let k = i + j;
            minimum_cost[j][k] = std::usize::MAX;

            for l in minimum_pos[j][k - 1]..=minimum_pos[j + 1][k] {
                let cost = minimum_cost[j][l] + minimum_cost[l][k];

                if cost < minimum_cost[j][k] {
                    minimum_cost[j][k] = cost;
                    minimum_pos[j][k] = l;
                }
            }

            minimum_cost[j][k] += crop_pos[k] - crop_pos[j];

            j += 1;
        }
    }

    println!("{}", minimum_cost[0][m - 1]);
}
