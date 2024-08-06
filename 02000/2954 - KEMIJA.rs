use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let s = s.chars().collect::<Vec<_>>();
    let mut idx = 0;
    let mut ret = String::new();

    while idx < s.len() {
        ret.push(s[idx]);

        if idx + 2 < s.len()
            && (s[idx] == 'a' || s[idx] == 'e' || s[idx] == 'i' || s[idx] == 'o' || s[idx] == 'u')
            && s[idx + 1] == 'p'
            && s[idx] == s[idx + 2]
        {
            idx += 2;
        }
        
        idx += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
