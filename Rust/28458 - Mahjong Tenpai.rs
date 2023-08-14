use io::Write;
use std::{collections::HashMap, io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn can_zumo(hand: &Vec<String>, card: String) -> bool {
    let mut card_pool = hand.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    card_pool.push(card.as_str());
    card_pool.sort_unstable();

    if card_pool.chunks_exact(2).all(|c| c[0] == c[1])
        && card_pool
            .windows(4)
            .all(|w| w[0] != w[1] || w[1] != w[2] || w[2] != w[3])
    {
        return true;
    }

    if let Some(eq_pos) = card_pool.windows(2).position(|w| w[0] == w[1]) {
        let thirteen_orphans = [
            "1m", "1s", "1t", "9m", "9s", "9t", "b", "e", "h", "j", "n", "s", "w",
        ];

        if card_pool[..=eq_pos]
            .iter()
            .chain(card_pool[eq_pos + 2..].iter())
            .eq(thirteen_orphans.iter())
        {
            return true;
        }
    }

    let mut count = HashMap::new();

    for card in card_pool.iter() {
        *count.entry(*card).or_insert(0) += 1;
    }

    for (p, q) in count {
        if q >= 2 {
            let first = card_pool.iter().position(|x| *x == p).unwrap();
            card_pool.swap(0, first);

            let second = card_pool[1..].iter().position(|x| *x == p).unwrap() + 1;
            card_pool.swap(1, second);

            if is_torso(&mut card_pool[2..]) {
                return true;
            }
        }
    }

    false
}

fn is_torso(hand: &mut [&str]) -> bool {
    hand.sort_unstable();

    let Some((&mut first, rest)) = hand.split_first_mut() else {
        return true;
    };

    if first.len() == 2 {
        if let Some(second) = rest.iter().position(|&x| x == first) {
            rest.swap(0, second);

            if let Some(third) = rest[1..].iter().position(|&x| x == first) {
                rest.swap(1, third + 1);

                if is_torso(&mut rest[2..]) {
                    return true;
                }
            }
        }

        if first.as_bytes()[0] <= b'7' {
            let mut next = [first.as_bytes()[0] + 1, first.as_bytes()[1]];
            let s_next = unsafe { std::str::from_utf8_unchecked(&next) };

            if let Some(second) = rest.iter().position(|&x| x == s_next) {
                rest.swap(0, second);
                next[0] += 1;

                let s_next = unsafe { std::str::from_utf8_unchecked(&next) };

                if let Some(third) = rest.iter().position(|&x| x == s_next) {
                    rest.swap(1, third);

                    if is_torso(&mut rest[2..]) {
                        return true;
                    }
                }
            }
        }

        false
    } else {
        if let Some(second) = rest.iter().position(|&x| x == first) {
            rest.swap(0, second);

            if let Some(third) = rest[1..].iter().position(|&x| x == first) {
                rest.swap(1, third + 1);

                if is_torso(&mut rest[2..]) {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut cards = vec![
        "1s", "1s", "1s", "1s", "2s", "2s", "2s", "2s", "3s", "3s", "3s", "3s", "4s", "4s", "4s",
        "4s", "5s", "5s", "5s", "5s", "6s", "6s", "6s", "6s", "7s", "7s", "7s", "7s", "8s", "8s",
        "8s", "8s", "9s", "9s", "9s", "9s", "1t", "1t", "1t", "1t", "2t", "2t", "2t", "2t", "3t",
        "3t", "3t", "3t", "4t", "4t", "4t", "4t", "5t", "5t", "5t", "5t", "6t", "6t", "6t", "6t",
        "7t", "7t", "7t", "7t", "8t", "8t", "8t", "8t", "9t", "9t", "9t", "9t", "1m", "1m", "1m",
        "1m", "2m", "2m", "2m", "2m", "3m", "3m", "3m", "3m", "4m", "4m", "4m", "4m", "5m", "5m",
        "5m", "5m", "6m", "6m", "6m", "6m", "7m", "7m", "7m", "7m", "8m", "8m", "8m", "8m", "9m",
        "9m", "9m", "9m", "e", "e", "e", "e", "w", "w", "w", "w", "s", "s", "s", "s", "n", "n",
        "n", "n", "h", "h", "h", "h", "b", "b", "b", "b", "j", "j", "j", "j",
    ];
    let mut hand = vec![String::new(); 13];

    for i in 0..13 {
        hand[i] = scan.token::<String>();

        if let Some(pos) = cards.iter().position(|x| *x == hand[i]) {
            cards.remove(pos);
        }
    }

    cards.sort();
    cards.dedup();
    cards.retain(|card| can_zumo(&hand, card.to_string()));

    if cards.is_empty() {
        writeln!(out, "no tenpai").unwrap();
    } else {
        writeln!(out, "tenpai").unwrap();
        writeln!(out, "{}", cards.len()).unwrap();

        for card in cards {
            write!(out, "{card} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
