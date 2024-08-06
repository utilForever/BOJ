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
    let nums = input_integers();
    let n = nums[0] as usize;
    let m = nums[1] as usize;

    let mut board: Vec<Vec<char>> = vec![vec!['0'; m as usize]; n as usize];

    for i in 0..n {
        let mut j = 0;

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        for c in s.chars() {
            if j >= m {
                break;
            }

            board[i][j] = c;
            j += 1;
        }
    }

    let mut min_repaint = n * m;

    for i in 0..=n - 8 {
        for j in 0..=m - 8 {
            // Case 'W'
            let mut num_to_repaint = 0;
            let mut board_color = 'W';

            for k in i..i + 8 {
                for l in j..j + 8 {
                    if board[k][l] != board_color {
                        num_to_repaint += 1;
                    }

                    board_color = if board_color == 'W' { 'B' } else { 'W' };
                }

                board_color = if board_color == 'W' { 'B' } else { 'W' };
            }

            if min_repaint > num_to_repaint {
                min_repaint = num_to_repaint;
            }

            // Case 'B'
            num_to_repaint = 0;
            board_color = 'B';

            for k in i..i + 8 {
                for l in j..j + 8 {
                    if board[k][l] != board_color {
                        num_to_repaint += 1;
                    }

                    board_color = if board_color == 'W' { 'B' } else { 'W' };
                }

                board_color = if board_color == 'W' { 'B' } else { 'W' };
            }

            if min_repaint > num_to_repaint {
                min_repaint = num_to_repaint;
            }
        }
    }

    println!("{}", min_repaint);
}
