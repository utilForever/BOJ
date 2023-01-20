use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut cities = Vec::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let mut iter = s.split_whitespace();
        let city = iter.next().unwrap().parse::<String>().unwrap();
        let temperature = iter.next().unwrap().parse::<i64>().unwrap();

        cities.push((city, temperature));
    }

    cities.sort_by(|a, b| a.1.cmp(&b.1));

    writeln!(out, "{}", cities[0].0).unwrap();
}
