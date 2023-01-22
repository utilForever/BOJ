use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    let mut code = vec![String::new(); 5];

    for i in 0..5 {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();

        code[i] = s.clone();
    }

    let len = match s.len() {
        7..=11 => 2,
        12..=15 => 3,
        16..=19 => 4,
        20..=23 => 5,
        24..=27 => 6,
        28..=31 => 7,
        32..=35 => 8,
        _ => unreachable!(),
    };

    let mut code_new = vec![vec![' '; 15]; len];

    for i in 0..len {
        for j in 0..5 {
            for k in 0..3 {
                code_new[i][j * 3 + k] = code[j].chars().nth(i * 4 + k).unwrap();
            }
        }
    }

    let mut code_converted = vec![0; len];

    for i in 0..len {
        code_converted[i] = match code_new[i][..] {
            ['*', '*', '*', '*', ' ', '*', '*', ' ', '*', '*', ' ', '*', '*', '*', '*'] => 0,
            [' ', ' ', '*', ' ', ' ', '*', ' ', ' ', '*', ' ', ' ', '*', ' ', ' ', '*'] => 1,
            ['*', '*', '*', ' ', ' ', '*', '*', '*', '*', '*', ' ', ' ', '*', '*', '*'] => 2,
            ['*', '*', '*', ' ', ' ', '*', '*', '*', '*', ' ', ' ', '*', '*', '*', '*'] => 3,
            ['*', ' ', '*', '*', ' ', '*', '*', '*', '*', ' ', ' ', '*', ' ', ' ', '*'] => 4,
            ['*', '*', '*', '*', ' ', ' ', '*', '*', '*', ' ', ' ', '*', '*', '*', '*'] => 5,
            ['*', '*', '*', '*', ' ', ' ', '*', '*', '*', '*', ' ', '*', '*', '*', '*'] => 6,
            ['*', '*', '*', ' ', ' ', '*', ' ', ' ', '*', ' ', ' ', '*', ' ', ' ', '*'] => 7,
            ['*', '*', '*', '*', ' ', '*', '*', '*', '*', '*', ' ', '*', '*', '*', '*'] => 8,
            ['*', '*', '*', '*', ' ', '*', '*', '*', '*', ' ', ' ', '*', '*', '*', '*'] => 9,
            _ => -1,
        };
    }

    if code_converted.iter().any(|&x| x == -1) {
        writeln!(out, "BOOM!!").unwrap();
        return;
    }

    let ret = code_converted.iter().fold(0, |acc, x| acc * 10 + x);
    writeln!(out, "{}", if ret % 6 == 0 { "BEER!!" } else { "BOOM!!" }).unwrap();
}
