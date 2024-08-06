use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut solution = String::new();
        io::stdin().read_line(&mut solution).unwrap();
        solution = solution.trim().to_string();

        if solution.is_empty() {
            break;
        }

        writeln!(out, "{solution}").unwrap();
    }
}
