use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    t = t.trim().to_string();

    let mut r = String::new();
    io::stdin().read_line(&mut r).unwrap();
    r = r.trim().to_string();

    let mut lcs_count = vec![vec![vec![0; 101]; 101]; 101];
    let mut i = 0;
    let mut j;
    let mut k;

    for s in s.chars() {
        i += 1;
        j = 0;

        for t in t.chars() {
            j += 1;
            k = 0;

            for r in r.chars() {
                k += 1;

                if s != t || t != r || s != r {
                    lcs_count[i][j][k] = *vec![
                        lcs_count[i - 1][j][k],
                        lcs_count[i][j - 1][k],
                        lcs_count[i][j][k - 1],
                    ]
                    .iter()
                    .max()
                    .unwrap();
                } else {
                    lcs_count[i][j][k] = lcs_count[i - 1][j - 1][k - 1] + 1;
                }
            }
        }
    }

    println!(
        "{}",
        lcs_count[s.chars().count()][t.chars().count()][r.chars().count()]
    );
}
