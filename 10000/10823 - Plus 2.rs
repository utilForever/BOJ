use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();

    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        temp = temp.trim().to_string();

        if temp.is_empty() {
            break;
        }

        s.push_str(&temp);
    }

    let nums = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    writeln!(out, "{}", nums.iter().sum::<i64>()).unwrap();
}
