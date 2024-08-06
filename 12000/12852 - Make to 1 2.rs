use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut cnt = vec![0; 1000001];
    let mut track = vec![Vec::new(); 1000001];

    cnt[1] = 0;
    track[1].push(1);
    cnt[2] = 1;
    track[2].push(2);
    track[2].push(1);
    cnt[3] = 1;
    track[3].push(3);
    track[3].push(1);

    for i in 4..=n {
        if i % 2 == 0 && i % 3 == 0 {
            cnt[i] = vec![cnt[i / 2], cnt[i / 3], cnt[i - 1]]
                .iter()
                .min()
                .unwrap()
                + 1;

            if cnt[i] == cnt[i / 2] + 1 {
                track[i] = track[i / 2].clone();
            } else if cnt[i] == cnt[i / 3] + 1 {
                track[i] = track[i / 3].clone();
            } else {
                track[i] = track[i - 1].clone();
            }
        } else if i % 2 == 0 {
            if cnt[i / 2] > cnt[i - 1] {
                cnt[i] = cnt[i - 1] + 1;
                track[i] = track[i - 1].clone();
            } else {
                cnt[i] = cnt[i / 2] + 1;
                track[i] = track[i / 2].clone();
            }
        } else if i % 3 == 0 {
            if cnt[i / 3] > cnt[i - 1] {
                cnt[i] = cnt[i - 1] + 1;
                track[i] = track[i - 1].clone();
            } else {
                cnt[i] = cnt[i / 3] + 1;
                track[i] = track[i / 3].clone();
            }
        } else {
            cnt[i] = cnt[i - 1] + 1;
            track[i] = track[i - 1].clone();
        }

        track[i].insert(0, i);
    }

    println!("{}", cnt[n]);
    for num in track[n].iter() {
        print!("{} ", num);
    }
    println!();
}
