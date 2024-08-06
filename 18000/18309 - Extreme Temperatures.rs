use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut lowest = i64::MAX;
    let mut highest = i64::MIN;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let data = s.split_whitespace().collect::<Vec<_>>();

        for i in 1..data.len() {
            let temperature = data[i].parse::<i64>().unwrap();

            lowest = lowest.min(temperature);
            highest = highest.max(temperature);
        }
    }

    writeln!(out, "{lowest} {highest}").unwrap();
}
