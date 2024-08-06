use io::Write;
use std::io;

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

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

        let s = s.chars().collect::<Vec<_>>();

        if !s.contains(&'.') {
            writeln!(out, "{} = {} / 1", s.iter().collect::<String>(), s.iter().collect::<String>()).unwrap();
            return;
        }

        let pos = s.iter().position(|&x| x == '.').unwrap();
        let number = s[0..pos].iter().collect::<String>().parse::<i64>().unwrap();
        let s_orig = s.clone();
        let s = &s[pos - 1..];

        if s[2..].contains(&'(') {
            // Indefinitely case
            let pos = s[2..].iter().position(|&x| x == '(').unwrap();
            let prefix = if pos == 0 {
                0
            } else {
                s[2..pos + 2]
                    .iter()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            };
            let repeating = s[pos + 3..s.len() - 1]
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            let len_prefix = pos as u32;
            let len_repeating = (s.len() - 3 - pos) as u32;

            let numerator = prefix * 10i64.pow(len_repeating - 1) + repeating - prefix;
            let denominator = 10i64.pow(len_prefix + len_repeating - 1) - 10i64.pow(len_prefix);
            let gcd = gcd(numerator, denominator);

            writeln!(
                out,
                "{} = {} / {}",
                s_orig.iter().collect::<String>(),
                number * (denominator / gcd) + numerator / gcd,
                denominator / gcd
            )
            .unwrap();
        } else {
            // Finitely case
            let digit = s[2..].len();
            let numerator = s[2..].iter().collect::<String>().parse::<i64>().unwrap();
            let denominator = 10i64.pow(digit as u32);
            let gcd = gcd(numerator, denominator);

            writeln!(
                out,
                "{} = {} / {}",
                s_orig.iter().collect::<String>(),
                number * (denominator / gcd) + numerator / gcd,
                denominator / gcd
            )
            .unwrap();
        }
    }
}
