use io::Write;
use std::io;

fn preprocess(s: &String) -> Vec<usize> {
    let s_chars = s.as_bytes();
    let s_len = s_chars.len();

    let mut pi = vec![0; s_len];
    let mut j = 0;

    for i in 1..s_len {
        while j > 0 && s_chars[i] != s_chars[j] {
            j = pi[j - 1];
        }

        if s_chars[i] == s_chars[j] {
            pi[i] = j + 1;
            j += 1;
        } else {
            pi[i] = 0;
        }
    }

    pi
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let s_len = s.len();
    let pi = preprocess(&s);

    let mut count = vec![0; s_len + 1];

    for i in 0..s_len {
        count[pi[i]] += 1;
    }

    for i in (1..=s_len).rev() {
        count[pi[i - 1]] += count[i];
    }

    let mut ans = Vec::new();
    let mut i = s_len;

    while i > 0 {
        ans.push((i, count[i] + 1));
        i = pi[i - 1];
    }

    ans.reverse();

    writeln!(out, "{}", ans.len()).unwrap();
    for (i, j) in ans {
        writeln!(out, "{} {}", i, j).unwrap();
    }
}
