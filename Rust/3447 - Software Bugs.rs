use io::Write;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let mut line = line.unwrap();

        while line.contains("BUG") {
            line = line.replace("BUG", "");
        }
        
        writeln!(out, "{line}").unwrap();
    }

    Ok(())
}
