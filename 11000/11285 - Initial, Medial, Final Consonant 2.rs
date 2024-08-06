use io::Write;
use std::io;

fn main() {
    let first = [
        'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ',
        'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];
    let middle = [
        'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ',
        'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
    ];
    let last = [
        ' ', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ',
        'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut str_first = String::new();
    io::stdin().read_line(&mut str_first).unwrap();
    let str_first = str_first.chars().next().unwrap();
    let idx_first = first.iter().position(|&c| c == str_first).unwrap() as u32;

    let mut str_middle = String::new();
    io::stdin().read_line(&mut str_middle).unwrap();
    let str_middle = str_middle.chars().next().unwrap();
    let idx_middle = middle.iter().position(|&c| c == str_middle).unwrap() as u32;

    let mut str_last = String::new();
    io::stdin().read_line(&mut str_last).unwrap();
    let str_last = str_last.chars().next().unwrap();

    let idx_last = match last.iter().position(|&c| c == str_last) {
        Some(idx) => idx as u32,
        None => 0,
    };

    writeln!(
        out,
        "{}",
        char::from_u32(0xAC00 + 588 * idx_first + 28 * idx_middle + idx_last).unwrap()
    )
    .unwrap();
}
