use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let mut accumulated_file_size = vec![0; 5001];
    let mut minimum_index = vec![vec![0; 5001]; 5001];
    let mut minimum_cost = vec![vec![0; 5001]; 5001];

    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let k = input_integers()[0] as usize;
        let file_size = input_integers();

        for i in 1..=k {
            accumulated_file_size[i] = accumulated_file_size[i - 1] + file_size[i - 1];
            minimum_index[i][i] = i;
        }

        for i in 1..=k {
            let mut j = 1;

            while i + j <= k {
                let l = i + j;
                minimum_cost[j][l] = std::i32::MAX;

                for m in minimum_index[j][l - 1]..=minimum_index[j + 1][l] {
                    let cost1 = minimum_cost[j][m];
                    let cost2 = if m + 1 >= k {
                        0
                    } else {
                        minimum_cost[m + 1][l]
                    };
                    let cost =
                        cost1 + cost2 + accumulated_file_size[l] - accumulated_file_size[j - 1];

                    if minimum_cost[j][l] > cost {
                        minimum_cost[j][l] = cost;
                        minimum_index[j][l] = m;
                    }
                }

                j += 1;
            }
        }

        println!("{}", minimum_cost[1][k]);
    }
}
