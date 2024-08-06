use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let s = s.chars().collect::<Vec<_>>();

    let ucpc = ['U', 'C', 'P', 'C'];
    let mut cnt = 0;
    let mut ret = false;

    for c in s {
        if c == ucpc[cnt] {
            cnt += 1;

            if cnt == 4 {
                ret = true;
                break;
            }
        }
    }

    writeln!(out, "{}", if ret { "I love UCPC" } else { "I hate UCPC" }).unwrap();
}
