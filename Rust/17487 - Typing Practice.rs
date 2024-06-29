use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let left = [
        'Q', 'W', 'E', 'R', 'T', 'Y', 'A', 'S', 'D', 'F', 'G', 'Z', 'X', 'C', 'V', 'B',
    ];
    let right = ['U', 'I', 'O', 'P', 'H', 'J', 'K', 'L', 'N', 'M'];

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut cnt_left = 0;
    let mut cnt_right = 0;
    let mut cnt_unknown = 0;

    for c in s.chars() {
        if left.contains(&c.to_ascii_uppercase()) {
            cnt_left += 1;
        } else if right.contains(&c.to_ascii_uppercase()) {
            cnt_right += 1;
        } else {
            cnt_unknown += 1;
        }

        if c.is_uppercase() {
            cnt_unknown += 1;
        }
    }

    if cnt_left > cnt_right {
        let val = (cnt_left - cnt_right).min(cnt_unknown);
        cnt_right += val;
        cnt_unknown -= val;
    } else if cnt_right > cnt_left {
        let val = (cnt_right - cnt_left).min(cnt_unknown);
        cnt_left += val;
        cnt_unknown -= val;
    }

    cnt_left += cnt_unknown / 2;
    cnt_right += cnt_unknown / 2;

    if cnt_unknown % 2 == 1 {
        cnt_left += 1;
    }

    writeln!(out, "{cnt_left} {cnt_right}").unwrap();
}
