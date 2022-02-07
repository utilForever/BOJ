use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    let mut p = String::new();
    io::stdin().read_line(&mut p).unwrap();

    let p_chars = p.as_bytes();
    let t_chars = t.as_bytes();
    let p_len = p_chars.len();
    let t_len = t_chars.len();

    let mut cmp = 0;
    let mut fail = vec![0; 1_000_000];

    for i in 1..p_len - 1 {
        while cmp > 0 && p_chars[cmp] != p_chars[i] {
            cmp = fail[cmp - 1];
        }

        if p_chars[cmp] == p_chars[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    let mut result = Vec::new();
    cmp = 0;

    for i in 0..t_len - 1 {
        while cmp > 0 && t_chars[i] != p_chars[cmp] {
            cmp = fail[cmp - 1];
        }

        if t_chars[i] == p_chars[cmp] {
            if cmp == p_len - 2 {
                result.push(i - cmp + 1);
                cmp = fail[cmp];
            } else {
                cmp += 1;
            }
        }
    }

    println!("{}", result.len());
    for val in result {
        print!("{} ", val);
    }

    println!("");
}
