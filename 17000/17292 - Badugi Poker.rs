use io::Write;
use std::{io, str};

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

fn compare_cards(
    a: &((char, char), (char, char)),
    b: &((char, char), (char, char)),
) -> std::cmp::Ordering {
    let is_a_same_color = a.0 .1 == a.1 .1;
    let is_b_same_color = b.0 .1 == b.1 .1;
    let number_a_1 = if a.0 .0 >= 'a' && a.0 .0 <= 'f' {
        a.0 .0 as u8 - 'a' as u8 + 10
    } else {
        a.0 .0 as u8 - '0' as u8
    };
    let number_a_2 = if a.1 .0 >= 'a' && a.1 .0 <= 'f' {
        a.1 .0 as u8 - 'a' as u8 + 10
    } else {
        a.1 .0 as u8 - '0' as u8
    };
    let number_b_1 = if b.0 .0 >= 'a' && b.0 .0 <= 'f' {
        b.0 .0 as u8 - 'a' as u8 + 10
    } else {
        b.0 .0 as u8 - '0' as u8
    };
    let number_b_2 = if b.1 .0 >= 'a' && b.1 .0 <= 'f' {
        b.1 .0 as u8 - 'a' as u8 + 10
    } else {
        b.1 .0 as u8 - '0' as u8
    };

    let number_a_max = number_a_1.max(number_a_2);
    let number_a_min = number_a_1.min(number_a_2);
    let number_b_max = number_b_1.max(number_b_2);
    let number_b_min = number_b_1.min(number_b_2);

    // Case 1: Same color
    if is_a_same_color && !is_b_same_color {
        return std::cmp::Ordering::Less;
    } else if !is_a_same_color && is_b_same_color {
        return std::cmp::Ordering::Greater;
    }

    // Case 2: Max number is bigger
    if number_a_max > number_b_max {
        return std::cmp::Ordering::Less;
    } else if number_a_max < number_b_max {
        return std::cmp::Ordering::Greater;
    }

    // Case 3: Min number is bigger
    if number_a_min > number_b_min {
        return std::cmp::Ordering::Less;
    } else if number_a_min < number_b_min {
        return std::cmp::Ordering::Greater;
    }

    // Case 4: The color of card of max number is black
    let is_a_max_black = if (number_a_max == number_a_1 && a.0 .1 == 'b')
        || (number_a_max == number_a_2 && a.1 .1 == 'b')
    {
        true
    } else {
        false
    };
    let is_b_max_black = if (number_b_max == number_b_1 && b.0 .1 == 'b')
        || (number_b_max == number_b_2 && b.1 .1 == 'b')
    {
        true
    } else {
        false
    };

    if is_a_max_black && !is_b_max_black {
        return std::cmp::Ordering::Less;
    } else if !is_a_max_black && is_b_max_black {
        return std::cmp::Ordering::Greater;
    }

    std::cmp::Ordering::Equal
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let mut cards = Vec::new();

    s.split(',').for_each(|card| {
        let card = card.to_string();
        let card = card.chars().collect::<Vec<_>>();
        cards.push((card[0], card[1]));
    });

    let mut cards1 = Vec::new();
    let mut cards2 = Vec::new();
    let mut cards3 = Vec::new();

    for i in 0..6 {
        for j in i + 1..6 {
            let num_card1 = if cards[i].0 >= 'a' && cards[i].0 <= 'f' {
                cards[i].0 as u8 - 'a' as u8 + 10
            } else {
                cards[i].0 as u8 - '0' as u8
            };
            let num_card2 = if cards[j].0 >= 'a' && cards[j].0 <= 'f' {
                cards[j].0 as u8 - 'a' as u8 + 10
            } else {
                cards[j].0 as u8 - '0' as u8
            };

            // Case 1: Continuous number
            if num_card1 == num_card2 + 1
                || num_card2 == num_card1 + 1
                || (num_card1 == 1 && num_card2 == 15)
                || (num_card1 == 15 && num_card2 == 1)
            {
                cards1.push((cards[i], cards[j]));
                continue;
            }

            // Case 2: Same number
            if num_card1 == num_card2 {
                cards2.push((cards[i], cards[j]));
                continue;
            }

            // Case 3: Etc
            cards3.push((cards[i], cards[j]));
        }
    }

    // Sort each case
    cards1.sort_by(|a, b| compare_cards(a, b));
    cards2.sort_by(|a, b| compare_cards(a, b));
    cards3.sort_by(|a, b| compare_cards(a, b));

    let ret = cards1
        .iter()
        .chain(cards2.iter())
        .chain(cards3.iter())
        .collect::<Vec<_>>();

    for (card1, card2) in ret {
        writeln!(out, "{}{}{}{}", card1.0, card1.1, card2.0, card2.1).unwrap();
    }
}
