use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let s = s.chars().collect::<Vec<_>>();

    let mut cnt_happy = 0;
    let mut cnt_sad = 0;

    s.windows(3).for_each(|w| {
        if w[0] == ':' && w[1] == '-' {
            match w[2] {
                ')' => cnt_happy += 1,
                '(' => cnt_sad += 1,
                _ => (),
            }
        }
    });

    writeln!(
        out,
        "{}",
        if cnt_happy == 0 && cnt_sad == 0 {
            "none"
        } else if cnt_happy == cnt_sad {
            "unsure"
        } else if cnt_happy > cnt_sad {
            "happy"
        } else {
            "sad"
        }
    )
    .unwrap();
}
