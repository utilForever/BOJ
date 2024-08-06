use io::Write;
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::HashMap,
    io, str,
};

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
enum Category {
    HighCard(Hand),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Hand),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(mut cards: Vec<Card>) -> Self {
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        Self { cards }
    }

    fn get_category(&self) -> Category {
        let is_suit_all_same = self
            .cards
            .iter()
            .all(|card| card.suit == self.cards[0].suit);
        let is_straight_normal = self
            .cards
            .windows(2)
            .all(|window| window[0].rank as i64 == window[1].rank as i64 + 1);
        let is_straight_baby = self.cards[0].rank == Rank::Ace
            && self.cards[1].rank == Rank::Five
            && self.cards[2].rank == Rank::Four
            && self.cards[3].rank == Rank::Three
            && self.cards[4].rank == Rank::Two;
        let is_straight = is_straight_normal || is_straight_baby;

        let ranks: HashMap<Rank, i64> = self.cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card.rank).or_insert(0) += 1;
            acc
        });
        let mut ranks = ranks
            .iter()
            .map(|(rank, count)| (*rank, *count))
            .collect::<Vec<(Rank, i64)>>();
        ranks.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

        // Check straight flush
        if is_suit_all_same && is_straight {
            if self.cards[4].rank == Rank::Ten {
                return Category::RoyalFlush;
            } else if is_straight_baby {
                return Category::StraightFlush(Rank::Five);
            } else {
                return Category::StraightFlush(self.cards[0].rank);
            }
        }

        // Check four of a kind
        if ranks[0].1 == 4 {
            return Category::FourOfAKind(ranks[0].0, ranks[1].0);
        }

        // Check full house
        if ranks[0].1 == 3 && ranks[1].1 == 2 {
            return Category::FullHouse(ranks[0].0, ranks[1].0);
        }

        // Check flush
        if is_suit_all_same {
            return Category::Flush(self.clone());
        }

        // Check straight
        if is_straight {
            if is_straight_baby {
                return Category::Straight(ranks[1].0);
            } else {
                return Category::Straight(ranks[0].0);
            }
        }

        // Check three of a kind
        if ranks[0].1 == 3 {
            return Category::ThreeOfAKind(ranks[0].0, ranks[1].0, ranks[2].0);
        }

        // Check two pair
        if ranks[0].1 == 2 && ranks[1].1 == 2 {
            return Category::TwoPair(ranks[0].0, ranks[1].0, ranks[2].0);
        }

        // Check one pair
        if ranks[0].1 == 2 {
            return Category::OnePair(ranks[0].0, ranks[1].0, ranks[2].0, ranks[3].0);
        }

        // Check high card
        Category::HighCard(self.clone())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let convert = |category: Category| -> u8 {
            match category {
                Category::HighCard(_) => 1,
                Category::OnePair(_, _, _, _) => 2,
                Category::TwoPair(_, _, _) => 3,
                Category::ThreeOfAKind(_, _, _) => 4,
                Category::Straight(_) => 5,
                Category::Flush(_) => 6,
                Category::FullHouse(_, _) => 7,
                Category::FourOfAKind(_, _) => 8,
                Category::StraightFlush(_) => 9,
                Category::RoyalFlush => 10,
            }
        };

        let category = self.get_category();
        let other_category = other.get_category();
        let ret_compare = convert(category.clone()).cmp(&convert(other_category.clone()));

        match ret_compare {
            Ordering::Less | Ordering::Greater => Some(ret_compare),
            Ordering::Equal => {
                let ret = match (category, other_category) {
                    (Category::HighCard(hand), Category::HighCard(other_hand)) => {
                        Some(hand.cards.cmp(&other_hand.cards))
                    }
                    (
                        Category::OnePair(rank1, rank2, rank3, rank4),
                        Category::OnePair(other_rank1, other_rank2, other_rank3, other_rank4),
                    ) => Some(
                        rank1.cmp(&other_rank1).then(
                            rank2
                                .cmp(&other_rank2)
                                .then(rank3.cmp(&other_rank3).then(rank4.cmp(&other_rank4))),
                        ),
                    ),
                    (
                        Category::TwoPair(rank1, rank2, rank3),
                        Category::TwoPair(other_rank1, other_rank2, other_rank3),
                    ) => Some(
                        rank1
                            .cmp(&other_rank1)
                            .then(rank2.cmp(&other_rank2).then(rank3.cmp(&other_rank3))),
                    ),
                    (
                        Category::ThreeOfAKind(rank1, rank2, rank3),
                        Category::ThreeOfAKind(other_rank1, other_rank2, other_rank3),
                    ) => Some(
                        rank1
                            .cmp(&other_rank1)
                            .then(rank2.cmp(&other_rank2).then(rank3.cmp(&other_rank3))),
                    ),
                    (Category::Straight(rank), Category::Straight(other_rank)) => {
                        Some(rank.cmp(&other_rank))
                    }
                    (Category::Flush(hand), Category::Flush(other_hand)) => {
                        Some(hand.cards.cmp(&other_hand.cards))
                    }
                    (
                        Category::FullHouse(rank1, rank2),
                        Category::FullHouse(other_rank1, other_rank2),
                    ) => Some(rank1.cmp(&other_rank1).then(rank2.cmp(&other_rank2))),
                    (
                        Category::FourOfAKind(rank1, rank2),
                        Category::FourOfAKind(other_rank1, other_rank2),
                    ) => Some(rank1.cmp(&other_rank1).then(rank2.cmp(&other_rank2))),
                    (Category::StraightFlush(rank), Category::StraightFlush(other_rank)) => {
                        Some(rank.cmp(&other_rank))
                    }
                    (Category::RoyalFlush, Category::RoyalFlush) => Some(Ordering::Equal),
                    _ => None,
                };

                ret.filter(|compare| matches!(compare, Ordering::Less | Ordering::Greater))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Decision {
    Alice,
    Bob,
    Draw,
}

fn check_result(
    cards_alice: &Vec<Card>,
    cards_bob: &Vec<Card>,
    cards_community: &Vec<Card>,
    selected_alice: &mut [usize; 3],
    selected_bob: &mut [usize; 3],
) -> Decision {
    let hands_alice = Hand::new(
        [
            cards_alice[0].clone(),
            cards_alice[1].clone(),
            cards_community[selected_alice[0]].clone(),
            cards_community[selected_alice[1]].clone(),
            cards_community[selected_alice[2]].clone(),
        ]
        .to_vec(),
    );
    let hands_bob = Hand::new(
        [
            cards_bob[0].clone(),
            cards_bob[1].clone(),
            cards_community[selected_bob[0]].clone(),
            cards_community[selected_bob[1]].clone(),
            cards_community[selected_bob[2]].clone(),
        ]
        .to_vec(),
    );

    match hands_alice.partial_cmp(&hands_bob) {
        Some(order) => match order {
            Ordering::Less => Decision::Bob,
            Ordering::Equal => Decision::Draw,
            Ordering::Greater => Decision::Alice,
        },
        None => Decision::Draw,
    }
}

fn process_dfs(
    cards_alice: &Vec<Card>,
    cards_bob: &Vec<Card>,
    cards_community: &Vec<Card>,
    selected_alice: &mut [usize; 3],
    selected_bob: &mut [usize; 3],
    visited: &mut Vec<bool>,
    num_turn: usize,
) -> Decision {
    if num_turn == 7 {
        return check_result(
            cards_alice,
            cards_bob,
            cards_community,
            selected_alice,
            selected_bob,
        );
    }

    let mut ret = [0; 3];

    if num_turn % 2 == 1 {
        for i in 0..6 {
            if !visited[i] {
                selected_alice[num_turn / 2] = i;
                visited[i] = true;

                let decision = process_dfs(
                    cards_alice,
                    cards_bob,
                    cards_community,
                    selected_alice,
                    selected_bob,
                    visited,
                    num_turn + 1,
                );

                ret[decision.clone() as usize] += 1;
                visited[i] = false;

                if decision == Decision::Alice {
                    break;
                }
            }
        }
    } else {
        for i in 0..6 {
            if !visited[i] {
                selected_bob[(num_turn - 1) / 2] = i;
                visited[i] = true;

                let decision = process_dfs(
                    cards_alice,
                    cards_bob,
                    cards_community,
                    selected_alice,
                    selected_bob,
                    visited,
                    num_turn + 1,
                );

                ret[decision.clone() as usize] += 1;
                visited[i] = false;

                if decision == Decision::Bob {
                    break;
                }
            }
        }
    }

    if num_turn % 2 == 1 {
        if ret[Decision::Alice as usize] > 0 {
            return Decision::Alice;
        }

        if ret[Decision::Draw as usize] > 0 {
            return Decision::Draw;
        }

        return Decision::Bob;
    } else {
        if ret[Decision::Bob as usize] > 0 {
            return Decision::Bob;
        }

        if ret[Decision::Draw as usize] > 0 {
            return Decision::Draw;
        }

        return Decision::Alice;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let convert_card = |raw_card: &str| -> Card {
        let suit = match &raw_card[1..2] {
            "D" => Suit::Diamond,
            "C" => Suit::Club,
            "H" => Suit::Heart,
            "S" => Suit::Spade,
            _ => panic!("Invalid suit"),
        };
        let rank = match &raw_card[0..1] {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "T" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => panic!("Invalid rank"),
        };

        Card { rank, suit }
    };

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut cards_alice = Vec::new();
        let mut cards_bob = Vec::new();
        let mut cards_community = Vec::new();

        for _ in 0..2 {
            cards_alice.push(convert_card(&scan.token::<String>()));
        }

        for _ in 0..2 {
            cards_bob.push(convert_card(&scan.token::<String>()));
        }

        for _ in 0..6 {
            cards_community.push(convert_card(&scan.token::<String>()));
        }

        let mut visited = vec![false; 6];
        let mut selected_alice = [0; 3];
        let mut selected_bob = [0; 3];
        let ret = process_dfs(
            &cards_alice,
            &cards_bob,
            &cards_community,
            &mut selected_alice,
            &mut selected_bob,
            &mut visited,
            1,
        );

        writeln!(
            out,
            "{}",
            match ret {
                Decision::Alice => "Alice",
                Decision::Bob => "Bob",
                Decision::Draw => "Draw",
            }
        )
        .unwrap();
    }
}
