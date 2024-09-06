//! [aoc](https://adventofcode.com/2023/day/7)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::BTreeMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 7,
        title: "Camel Cards",
        solution: ("251121738", "251421071"),
        example_solutions: vec![("6440", "5905")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut hands = Vec::new();
    for &line in input {
        let mut a_iter = line.split(' ');
        let hand = a_iter.next().unwrap();
        let bid = a_iter
            .next()
            .ok_or("missing bid")?
            .parse::<ItemType>()
            .map_err(|_| "bid must be integer")?;
        if a_iter.next().is_some() {
            Err("lines must contain only hand and bid")?;
        }
        hands.push(CamelHand::new(hand, bid)?);
    }
    // ---------- Part 1
    let mut ans1 = 0;
    hands.sort_by_key(|x| x.strength);
    for (idx, hand) in hands.iter().enumerate() {
        ans1 += (idx as ItemType + 1) * hand.bid;
    }
    // ---------- Part 2
    let mut ans2 = 0;
    let mut joker_hands = hands
        .iter()
        .map(JokerHand::new)
        .collect::<Result<Vec<_>, PuzzleError>>()?;
    joker_hands.sort_by_key(|x| x.strength);
    for (idx, hand) in joker_hands.iter().enumerate() {
        ans2 += (idx as ItemType + 1) * hand.bid;
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

const CAMEL_LABELS: &str = "23456789TJQKA";

struct CamelHand {
    cards: String,
    bid: ItemType,
    hand_type: HandType,
    strength: ItemType,
}

impl CamelHand {
    fn new(cards: &str, bid: ItemType) -> Result<Self, PuzzleError> {
        if cards.len() != 5 {
            Err("hand must contain 5 cards")?;
        }
        let mut card_counts = BTreeMap::new();
        for c in cards.chars() {
            *card_counts.entry(c).or_insert(0) += 1;
        }
        let max_count = *card_counts
            .iter()
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .1;
        let hand_type = match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if max_count == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if max_count == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPairs
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => Err("impossible")?,
        };
        let mut strength = hand_type.clone() as ItemType;
        for c in cards.chars() {
            let p = CAMEL_LABELS.find(c).ok_or("invalid card")?;
            strength = (strength << 4) + p as ItemType;
        }
        Ok(Self {
            cards: cards.to_owned(),
            bid,
            hand_type,
            strength,
        })
    }
}

const JOKER_LABELS: &str = "J23456789TQKA";

#[allow(dead_code)]
struct JokerHand {
    cards: String,
    bid: ItemType,
    hand_type: HandType,
    strength: ItemType,
}

impl JokerHand {
    fn new(camel_hand: &CamelHand) -> Result<Self, PuzzleError> {
        let mut hand_type = HandType::HighCard;
        for i in 0..JOKER_LABELS.len() {
            let cards = camel_hand.cards.replace('J', &JOKER_LABELS[i..=i]);
            let hand = CamelHand::new(&cards, 0)?;
            if hand.hand_type.clone() as ItemType > hand_type.clone() as ItemType {
                hand_type = hand.hand_type.clone();
            }
        }
        let mut strength = hand_type.clone() as ItemType;
        for c in camel_hand.cards.chars() {
            let p = JOKER_LABELS.find(c).ok_or("invalid card")?;
            strength = (strength << 4) + p as ItemType;
        }
        Ok(Self {
            cards: camel_hand.cards.to_owned(),
            bid: camel_hand.bid,
            hand_type,
            strength,
        })
    }
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_missing_bid() {
        test_invalid(&[&"32T3K"], solve);
    }

    #[test]
    fn invalid_must_be_only_hand_and_bid() {
        test_invalid(&[&"32T3K 765 a"], solve);
    }

    #[test]
    fn invalid_bid_must_be_integer() {
        test_invalid(&[&"32T3K a"], solve);
    }

    #[test]
    fn invalid_must_contain_5_cards() {
        test_invalid(&[&"32T32K 765"], solve);
    }

    #[test]
    fn invalid_card() {
        test_invalid(&[&"32Z3K 765"], solve);
    }
}
