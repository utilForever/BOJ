use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut sum_euros = 0;
    let mut sum_cents = 0;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let mut parts = s.split_terminator('.');

        let euros = parts.next().unwrap().parse::<i64>().unwrap();
        let cents = parts.next().unwrap().parse::<i64>().unwrap();

        sum_euros += euros;
        sum_cents += cents;

        if sum_cents >= 100 {
            sum_euros += 1;
            sum_cents -= 100;
        }
    }

    writeln!(out, "{sum_euros}.{:02}", sum_cents).unwrap();
}
