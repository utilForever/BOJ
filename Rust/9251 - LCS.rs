use std::{cmp, io};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    t = t.trim().to_string();

    let mut lcs_count = vec![vec![0; 1001]; 1001];
    let mut i = 0;
    let mut j;

    for s in s.chars() {
        i += 1;
        j = 0;

        for t in t.chars() {
            j += 1;

            if s != t {
                lcs_count[i][j] = cmp::max(lcs_count[i - 1][j], lcs_count[i][j - 1]);
            } else {
                lcs_count[i][j] = lcs_count[i - 1][j - 1] + 1;
            }
        }
    }

    println!("{}", lcs_count[s.chars().count()][t.chars().count()]);
}
