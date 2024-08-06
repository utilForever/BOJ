use io::Write;
use std::{
    collections::HashMap,
    io::{self, BufWriter, StdoutLock},
};

type Out<'a> = BufWriter<StdoutLock<'a>>;

fn process0(out: &mut Out) {
    writeln!(out, "BOJ 20000\n").ok();
}

fn process1(out: &mut Out) {
    writeln!(out, "#include <cstdio>\nint main(){{\n    int N;\n    scanf(\"%d\",&N);\n    if(N==1){{\n        puts(\"4\");\n    }}").ok();

    for i in 2..=20000 {
        writeln!(
            out,
            "    else if(N=={}){{\n        puts(\"{}\");\n    }}",
            i,
            i * 4
        )
        .ok();
    }

    writeln!(
        out,
        "    else{{\n        puts(\"Still working on it...\");\n    }}\n    return 0;\n}}"
    )
    .ok();
}

fn process2(out: &mut Out) {
    let s = "BaekjoonOnlineJudge!".chars().collect::<Vec<_>>();

    for i in 1..1048576 {
        let mut idx = 0;
        let mut j = i;

        while j % 2 == 0 {
            j /= 2;
            idx += 1;
        }

        write!(out, "{}", s[idx]).ok();
    }

    writeln!(out).ok();
}

fn process3(out: &mut Out) {
    let leaf = "01000001101101110001110001011001000011000111110001011010100010101110111100101001000110001110001000011011111001000010101011100011010100011000010101101100000011100000110011101111011001010011000110110111001110111001111000101011001111010111010110001000101010100101000110001001110101110110111100010001011110011110001110011011100010111011110101010101111100011100101001110101111101100110010111110110011010011001111110001110101110010100110100000100000100010000000000000000000000000000000000000000000000000000000000000000011110100101111100000011101001010111001011001111001010111011010111110000011000010001001111011110111101100111000111011101010000110100001100100110011000110100110110000101101001001011100111101010010110111010110101111111111111111101000100101001100011001000000111101100001000000100101000011101100100101101111011001101101111010010011010000010111001100010000100100011101011100100101000010110100011001000011111001010110100011110110111000010100101100100111100101000100011001011101001011001110100000101101011000011010111011101001101110010010101100001010101111100110111111110010110101011011000101111111101101111001000100100001110011011110111110111001011111111111111111111111111111111111111111111111111111111111111111011010001001011101100011101001010100101110101100010100110000011010011100001010101110000011011011010111000101000110010101001011111100010011001000111100001011101101011100110000111010110001001001011001101000000111010111010101010000100101011001101111011011110101101011000011000000000010000010110100111111101000111100010000011000001000101010111100000111000111101101011101100100100111111011101011101001010101100001001101000001001100101011101000000110100101110100011010110100110011011100101111000100001011001010110110101101110011101111100001001010110111010010101111100100111110010000010111110010000011101101001011010110011111101111000001101000111111000101111011111011111010111001001101101011010110000010101101101100010000100101111001110000010010011110011111001101011111100001101000111110111000011100111100001000101011110111011110000000001";
    let mut stems = (0..10)
        .map(|i| vec![0; 2_usize.pow(i + 1)])
        .collect::<Vec<_>>();

    stems.push(
        leaf.chars()
            .map(|v| v.to_digit(10).unwrap() as usize)
            .collect(),
    );

    let mut ret = (0..1024).map(|i| vec![' '; 1025 + i]).collect::<Vec<_>>();

    for i in (0..=9).rev() {
        for j in 0..2_usize.pow(i as u32 + 1) {
            stems[i][j] = stems[i + 1][2 * j] | stems[i + 1][2 * j + 1];
        }
    }

    for i in 0..=10 {
        for j in 0..2_usize.pow(i as u32 + 1) {
            if stems[i][j] == 0 {
                continue;
            }

            let (mut x, mut y, mut z) = (
                1024 * (2 * (j / 2) + 1) / 2_usize.pow(i as u32) - 1,
                1024 - 2_usize.pow(10 - i as u32),
                '/',
            );

            if j & 1 == 1 {
                x += 1;
                z = '\\';
            }

            for _ in 0..std::cmp::max(2_usize.pow(9 - i as u32), 1) {
                ret[y][x] = z;

                if j & 1 == 1 {
                    x += 1;
                } else {
                    x -= 1;
                }

                y += 1;
            }
        }
    }

    for val in ret {
        writeln!(out, "{}", val.iter().collect::<String>()).ok();
    }
}

fn process4(out: &mut Out) {
    let mut is_prime = vec![true; 1025];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;

    while i * i <= 1024 {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=1024).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    for i in 0..1000 {
        for j in 0..=i {
            write!(out, "{}", if is_prime[i ^ j] { '#' } else { '.' }).ok();
        }

        writeln!(out).ok();
    }
}

fn process5(out: &mut Out) {
    let s = "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";

    for _ in 0..200 {
        write!(out, "{s}").ok();
    }

    writeln!(out).ok();
}

fn internal1(ret: &mut Vec<String>, n: usize, m: usize) {
    let mut stack = Vec::new();
    let mut add = Vec::new();

    internal2(&mut stack, &mut add, m, 1, n, m);

    ret.extend(add);
}

fn internal2(
    stack: &mut Vec<usize>,
    add: &mut Vec<String>,
    cnt: usize,
    curr: usize,
    n: usize,
    m: usize,
) {
    if cnt == 0 {
        add.push(stack.iter().map(|v| v.to_string()).collect());
        return;
    }

    for next in 1..=n {
        if !stack.contains(&next)
            && (cnt == m
                || internal3(curr, next).is_none()
                || stack.contains(&internal3(curr, next).unwrap()))
        {
            stack.push(next);
            internal2(stack, add, cnt - 1, next, n, m);
            stack.pop();
        }
    }
}

fn internal3(start: usize, end: usize) -> Option<usize> {
    if start == end {
        return None;
    }

    let (a, b) = ((start - 1) / 3, (start - 1) % 3);
    let (c, d) = ((end - 1) / 3, (end - 1) % 3);

    if !((a ^ c) & 1 != 0 || (b ^ d) & 1 != 0) {
        return Some((a + c) / 2 * 3 + (b + d) / 2 + 1);
    }

    None
}

fn process6(out: &mut Out) {
    let mut ret = Vec::new();

    for val in 4..10 {
        internal1(&mut ret, 9, val);
    }

    for val in ret {
        write!(out, "{val} ").ok();
    }

    writeln!(out).ok();
}

fn process7(out: &mut Out) {
    let letters = (b'A'..=b'Z')
        .chain(b'a'..=b'z')
        .chain(b'0'..=b'9')
        .chain([b'+', b'/'])
        .map(|c| c as char)
        .collect::<Vec<_>>();
    let mut letters_map = HashMap::new();

    for (idx, val) in letters.iter().enumerate() {
        letters_map.insert(*val, idx);
    }

    let s = "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";
    let mut s_converted = Vec::new();

    for c in s.chars() {
        s_converted.extend((0..6).map(|v| (letters_map[&c] >> (5 - v)) & 1));
    }

    let convert = |ret: &mut Vec<Vec<char>>, dx: i64, dy: i64, x: usize, y: usize| {
        for i in 1..5 {
            ret[y + dy as usize * i][x + dx as usize * i] = '!';
        }
    };

    let dx = [0, 0, 1, -1, 1, 1, -1, -1];
    let dy = [1, -1, 0, 0, 1, -1, 1, -1];
    let mut offset = 0;
    let mut ret = vec![vec!['.'; 1025]; 1025];

    for i in (3..10).rev() {
        for j in 1..(2_u32.pow(10 - i)) {
            for k in 1..(2_u32.pow(10 - i)) {
                let x = (2_u32.pow(i) * j) as usize;
                let y = (2_u32.pow(i) * k) as usize;
                let d = 2_u32.pow(i) as usize - 3;

                if j % 2 == 0 || k % 2 == 0 {
                    continue;
                }

                let table = vec![
                    (x + d - 1, y, 1),
                    (x, y + 1 - d, 2),
                    (x + 1 - d, y, 1),
                    (x, y + 1 - d, 3),
                    (x + 1 - d, y, 0),
                    (x, y + d - 1, 3),
                    (x + d - 1, y, 0),
                    (x, y + d - 1, 2),
                ];

                for l in 0..d {
                    ret[y - l][x] = '!';
                    ret[y + l][x] = '!';
                    ret[y][x - l] = '!';
                    ret[y][x + l] = '!';
                }

                for l in 0..4 {
                    let idx = 2 * l + s_converted[offset + l];
                    convert(
                        &mut ret,
                        dx[table[idx].2],
                        dy[table[idx].2],
                        table[idx].0,
                        table[idx].1,
                    );
                }

                offset += 4;
            }
        }
    }

    let mut stack = Vec::new();
    stack.push((4, 4));

    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();

        for i in 0..8 {
            let nx = x as i64 + dx[i];
            let ny = y as i64 + dy[i];

            if nx < 0 || nx >= 1025 || ny < 0 || ny >= 1025 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if ret[nx][ny] == '#' || ret[nx][ny] == '?' {
                continue;
            }

            if ret[nx][ny] == '.' {
                ret[nx][ny] = '#';
                continue;
            }

            stack.push((nx, ny));
            ret[nx][ny] = '?';
        }
    }

    for i in 0..1025 {
        for j in 0..1025 {
            if ret[i][j] == '?' {
                ret[i][j] = '.';
            }
        }
    }

    for i in 3..1022 {
        let row = ret[i]
            .iter()
            .skip(3)
            .take(1019)
            .map(|v| *v)
            .collect::<Vec<_>>();
        writeln!(out, "{}", row.iter().collect::<String>()).ok();
    }
}

fn process8(out: &mut Out) {
    let mut vals = vec![1; 4782966];
    let letters = [
        (63, '/'),
        (61, '9'),
        (45, 't'),
        (7, 'H'),
        (56, '4'),
        (40, 'o'),
        (5, 'F'),
        (47, 'v'),
        (0, 'A'),
    ];
    let mut ret = Vec::new();

    for i in 0..4782966 {
        let mut flag = false;

        for j in 0..7 {
            if (i / 3_usize.pow(j + 7)) % 3 == (i / 3_usize.pow(j)) % 3
                && (i / 3_usize.pow(j)) % 3 == 1
            {
                flag = true;
                break;
            }
        }

        if flag {
            vals[i] = 0;
        }
    }

    for i in 0..797161 {
        let mut sum = 0;

        for j in 0..6 {
            sum += 2_u8.pow(5 - j) * vals[(6 * i + j) as usize];
        }

        if let Some(&(_, c)) = letters.iter().find(|&&(v, _)| v == sum) {
            ret.push(c);
        }
    }

    writeln!(out, "{}", ret.iter().collect::<String>()).ok();
}

fn process9(out: &mut Out) {
    for i in 0_i128..100000_i128 {
        write!(
            out,
            "{}",
            (376739550 * i.pow(4)
                + 28540875 * i.pow(3)
                + 565524465 * i.pow(2)
                + 855292706 * i
                + 20000)
                % 1695327975
        )
        .unwrap();

        if i != 99999 {
            write!(out, ",").unwrap();
        } else {
            writeln!(out).unwrap();
        }
    }
}

// NOTE: Thanks to people in Discord server '20000' for helping me with this problem.
fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let n = s.trim().parse::<i64>().unwrap();

    match n {
        0 => process0(&mut out),
        1 => process1(&mut out),
        2 => process2(&mut out),
        3 => process3(&mut out),
        4 => process4(&mut out),
        5 => process5(&mut out),
        6 => process6(&mut out),
        7 => process7(&mut out),
        8 => process8(&mut out),
        9 => process9(&mut out),
        _ => unreachable!(),
    }
}
