use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for i in 1000..=9999 {
        let mut a = 0;
        let mut num = i;

        while num > 0 {
            a += num % 10;
            num /= 10;
        }

        let mut b = 0;
        let mut num = i;
        
        while num > 0 {
            b += num % 12;
            num /= 12;
        }

        let mut c = 0;
        let mut num = i;

        while num > 0 {
            c += num % 16;
            num /= 16;
        }

        if a == b && b == c {
            writeln!(out, "{i}").unwrap();
        }
    }
}
