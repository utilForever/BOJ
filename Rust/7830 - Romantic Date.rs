use io::Write;
use std::{cmp::Ordering, io, str};

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

fn compare_cards(a: &str, b: &str) -> Ordering {
    let a_rank = a.chars().nth(0).unwrap();
    let a_suit = a.chars().nth(1).unwrap();
    let b_rank = b.chars().nth(0).unwrap();
    let b_suit = b.chars().nth(1).unwrap();

    let convert_rank = |rank: char| -> i32 {
        match rank {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'J' => 12,
            'T' => 11,
            _ => rank.to_digit(10).unwrap() as i32,
        }
    };

    let convert_suit = |suit: char| -> i32 {
        match suit {
            'S' => 4,
            'H' => 3,
            'C' => 2,
            'D' => 1,
            _ => 0,
        }
    };

    let a_rank = convert_rank(a_rank);
    let a_suit = convert_suit(a_suit);
    let b_rank = convert_rank(b_rank);
    let b_suit = convert_suit(b_suit);

    if a_rank == b_rank {
        b_suit.cmp(&a_suit)
    } else {
        b_rank.cmp(&a_rank)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let cards = vec![
        "AS", "AH", "AC", "AD", "KS", "KH", "KC", "KD", "QS", "QH", "QC", "QD", "JS", "JH", "JC",
        "JD", "TS", "TH", "TC", "TD", "9S", "9H", "9C", "9D", "8S", "8H", "8C", "8D", "7S", "7H",
        "7C", "7D", "6S", "6H", "6C", "6D", "5S", "5H", "5C", "5D", "4S", "4H", "4C", "4D", "3S",
        "3H", "3C", "3D", "2S", "2H", "2C", "2D",
    ];

    for _ in 0..t {
        let mut cards_wibowo = Vec::with_capacity(26);

        for _ in 0..26 {
            let card = scan.token::<String>();
            cards_wibowo.push(card);
        }

        cards_wibowo.sort_by(|a, b| compare_cards(a, b));

        let mut cards_girlfriend = cards
            .iter()
            .filter(|&card| !cards_wibowo.contains(&card.to_string()))
            .map(|&card| card)
            .collect::<Vec<_>>();

        cards_girlfriend.sort_by(|a, b| compare_cards(a, b));

        let mut ret = 0;

        for i in 0..26 {
            let card_wibowo = cards_wibowo[i].clone();

            // Find the card_wibowo can beat the card_girlfriend that has the highest rank
            let mut idx = -1;

            for j in 0..cards_girlfriend.len() {
                // Ordering::Less means card_wibowo can beat card_girlfriend[j]
                if compare_cards(&card_wibowo, &cards_girlfriend[j]) == Ordering::Less {
                    idx = j as i64;
                    break;
                }
            }

            // If card_wibowo can't beat any card_girlfriend, then remove the card_girlfriend that has the highest rank
            // Otherwise, remove the card_girlfriend that is highest rank and can be beaten by card_wibowo
            if idx == -1 {
                cards_girlfriend.remove(0);
            } else {
                cards_girlfriend.remove(idx as usize);
                ret += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
