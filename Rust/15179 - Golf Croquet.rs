use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut team1 = String::new();
    io::stdin().read_line(&mut team1).unwrap();
    team1 = team1.trim().to_string();

    let mut team2 = String::new();
    io::stdin().read_line(&mut team2).unwrap();
    team2 = team2.trim().to_string();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.chars().collect::<Vec<_>>();

    let mut sum_x = 0;
    let mut sum_y = 0;

    for (idx, c) in code.iter().enumerate() {
        match c {
            'H' => {
                if idx % 2 == 0 {
                    sum_x += 1;
                } else {
                    sum_y += 1;
                }
            }
            'D' => {
                if idx % 2 == 0 {
                    sum_x += if sum_x == 6 { 1 } else { 2 };
                } else {
                    sum_y += if sum_y == 6 { 1 } else { 2 };
                }
            }
            'O' => {
                if idx % 2 == 0 {
                    sum_y += 1;
                } else {
                    sum_x += 1;
                }
            }
            _ => {
                if sum_x >= 7 || sum_y >= 7 {
                    break;
                }
            }
        }

        if sum_x >= 7 || sum_y >= 7 {
            break;
        }
    }

    write!(out, "{} {sum_x} {} {sum_y}. ", team1.clone(), team2.clone()).unwrap();

    if sum_x >= 7 {
        writeln!(out, "{team1} has won.").unwrap();
    } else if sum_y >= 7 {
        writeln!(out, "{team2} has won.").unwrap();
    } else if sum_x > sum_y {
        writeln!(out, "{team1} is winning.").unwrap();
    } else if sum_x < sum_y {
        writeln!(out, "{team2} is winning.").unwrap();
    } else {
        writeln!(out, "All square.").unwrap();
    }
}
