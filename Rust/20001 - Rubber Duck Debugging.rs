use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut num_problems = 0;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "고무오리 디버깅 끝" {
            break;
        } else if s == "문제" {
            num_problems += 1;
        } else if s == "고무오리" {
            if num_problems == 0 {
                num_problems += 2;
            } else {
                num_problems -= 1;
            }
        }
    }

    writeln!(
        out,
        "{}",
        if num_problems == 0 {
            "고무오리야 사랑해"
        } else {
            "힝구"
        }
    )
    .unwrap();
}
