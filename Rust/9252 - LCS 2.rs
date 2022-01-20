use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    t = t.trim().to_string();

    let mut lcs_count = vec![vec![0; 1001]; 1001];
    let mut lcs_str = vec![vec![String::new(); 1001]; 1001];
    let mut i = 0;
    let mut j;

    for s in s.chars() {
        i += 1;
        j = 0;

        for t in t.chars() {
            j += 1;

            if s != t {
                if lcs_count[i - 1][j] > lcs_count[i][j - 1] {
                    lcs_count[i][j] = lcs_count[i - 1][j];
                    lcs_str[i][j] = lcs_str[i - 1][j].clone();
                } else {
                    lcs_count[i][j] = lcs_count[i][j - 1];
                    lcs_str[i][j] = lcs_str[i][j - 1].clone();
                }
            } else {
                lcs_count[i][j] = lcs_count[i - 1][j - 1] + 1;
                lcs_str[i][j] = lcs_str[i - 1][j - 1].clone() + &s.to_string();
            }
        }
    }

    println!("{}", lcs_count[s.chars().count()][t.chars().count()]);
    println!("{}", lcs_str[s.chars().count()][t.chars().count()]);
}
