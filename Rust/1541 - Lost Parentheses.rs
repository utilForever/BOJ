use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
            
    let mut num: i32 = 0;
    let mut sum: i32 = 0;
    let mut flag: i32 = 1;

    for c in s.chars() {
        if c == '+' {
            sum += num;
            num = 0;
        } else if c == '-' {
            flag = -1;

            sum += num;
            num = 0;
        } else {
            num = num * 10 + (c as i32 - '0' as i32) * flag;
        }
    }

    println!("{}", sum + num);
}
