use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let mut n = s.parse::<i64>().unwrap();

        if n == 0 {
            writeln!(out, "0 = 0 GSC").unwrap();
            continue;
        }

        let n_orig = n;
        let mut is_negative = false;
        let mut ret = String::new();

        if n < 0 {
            is_negative = true;
            n = -n;
        }

        while n > 0 {
            let val = n % 3;

            ret.push(match val {
                0 => '0',
                1 => {
                    if is_negative {
                        '-'
                    } else {
                        '1'
                    }
                }
                2 => {
                    if is_negative {
                        '1'
                    } else {
                        '-'
                    }
                }
                _ => unreachable!(),
            });

            n /= 3;

            if val == 2 {
                n += 1;
            }
        }

        ret = ret.chars().rev().collect::<String>();

        writeln!(out, "{n_orig} = {ret} GSC").unwrap();
    }
}
