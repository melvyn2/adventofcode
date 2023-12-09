use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Formatter, Write};

const CARD_CHARS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const CARDS: [Card; 13] = [
    Card::Ace,
    Card::King,
    Card::Queen,
    Card::Jack,
    Card::Tee,
    Card::Nine,
    Card::Eight,
    Card::Seven,
    Card::Six,
    Card::Five,
    Card::Four,
    Card::Three,
    Card::Two,
];

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Tee,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
impl Card {
    fn rank(&self) -> usize {
        CARDS
            .iter()
            .enumerate()
            .find_map(|(i, c)| {
                if c == self {
                    Some(CARDS.len() - i)
                } else {
                    None
                }
            })
            .unwrap()
    }
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        CARD_CHARS
            .iter()
            .zip(CARDS.iter())
            .find_map(|(&v, &r)| if value == v { Some(r) } else { None })
            .unwrap()
    }
}
impl From<Card> for char {
    fn from(value: Card) -> Self {
        CARD_CHARS
            .iter()
            .zip(CARDS.iter())
            .find_map(|(&r, &v)| if value == v { Some(r) } else { None })
            .unwrap()
    }
}
impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Hand {
    inner: [Card; 5],
}
impl Hand {
    fn new(v: String) -> Option<Self> {
        let inner = v
            .chars()
            .map(Card::from)
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;
        Some(Self { inner })
    }

    fn hand_type(&self) -> usize {
        let mut map = BTreeMap::new();
        for card in self.inner {
            map.insert(card, 1 + map.get(&card).unwrap_or(&0));
        }
        match map.len() {
            1 => 7,
            2 => match map.pop_first().unwrap().1 {
                1 | 4 => 6,
                2 | 3 => 5,
                _ => unreachable!(),
            },
            3 => match map.pop_first().unwrap().1 {
                1 => match map.pop_first().unwrap().1 {
                    1 | 3 => 4,
                    2 => 3,
                    _ => unreachable!(),
                },
                3 => 4,
                2 => 3,
                _ => unreachable!(),
            },
            4 => 2,
            5 => 1,
            _ => unreachable!(),
        }
    }
}
impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type = self.hand_type().cmp(&other.hand_type());
        if hand_type == Ordering::Equal {
            self.inner
                .iter()
                .zip(other.inner)
                .find_map(|(&s, o)| {
                    let ord = s.cmp(&o);
                    if ord != Ordering::Equal {
                        Some(ord)
                    } else {
                        None
                    }
                })
                .unwrap_or(Ordering::Equal)
        } else {
            hand_type
        }
    }
}
impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.inner {
            f.write_char(c.into())?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("input.txt");

    let hands = input
        .lines()
        .map(|l| {
            let (h_str, b_str) = l.split_once(' ').unwrap();
            let bet = b_str.parse::<usize>().unwrap();
            let hand = Hand::new(h_str.to_string()).unwrap();
            (hand, bet)
        })
        .collect::<BTreeMap<Hand, usize>>()
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (hand, bid))| {
            // println!(
            //     "hand {} bid {} rank {} = {}",
            //     hand,
            //     bid,
            //     idx + 1,
            //     (idx + 1) * bid
            // );
            acc + (idx + 1) * bid
        });
    dbg!(hands);
}
