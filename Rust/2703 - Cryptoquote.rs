use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<i64>().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let mut s = s.chars().collect::<Vec<_>>();

        let mut table = String::new();
        io::stdin().read_line(&mut table).unwrap();
        table = table.trim().to_string();

        let table = table.chars().collect::<Vec<_>>();

        for c in s.iter_mut() {
            if *c == ' ' {
                continue;
            }

            *c = table[*c as usize - 'A' as usize];
        }

        writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
    }
}
