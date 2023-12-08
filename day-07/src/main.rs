use phf::phf_map;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file_name = "input.txt";
    let mut result = Vec::new();

    for line in read_to_string(file_name).unwrap().lines() {
        result.push(line.to_string())
    }

    part1(&result);
    let end = Instant::now();
    println!("Part 1 took {} milliseconds", (end - start).as_millis());
    part2(&result);
    // print!("{:?}", result);
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Hand {
    hand_type: HandType,
    card_vals: CardVal,
    value: String,
    bid: i32,
}
#[derive(Eq, PartialEq, Debug)]
struct CardVal {
    card_vals: Vec<i32>,
}

impl Ord for CardVal {
    fn cmp(&self, other: &Self) -> Ordering {
        for (i, v) in self.card_vals.iter().enumerate() {
            if v > &other.card_vals[i as usize] {
                return Ordering::Greater;
            }
            if v < &other.card_vals[i as usize] {
                return Ordering::Less;
            }
            continue;
        }
        println!("EQUALL???? {:?}, {:?}", self.card_vals, other.card_vals);
        // panic!("We weren't supposed to get identical hands :`(");
        return Ordering::Equal;
    }
}

impl PartialOrd for CardVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HandType::FiveOfAKind => write!(f, "Five of a kind"),
            HandType::FourOfAKind => write!(f, "Four of a kind"),
            HandType::FullHouse => write!(f, "Full house"),
            HandType::ThreeOfAKind => write!(f, "Three of a kind"),
            HandType::TwoPair => write!(f, "Two Pair"),
            HandType::OnePair => write!(f, "One pair"),
            HandType::HighCard => write!(f, "High card"),
        }
    }
}

static CARDS_MAP: phf::Map<&'static str, &'static i32> = phf_map! {
    "2" => &1,
    "3" => &2,
    "4" => &3,
    "5" => &4,
    "6" => &5,
    "7" => &6,
    "8" => &7,
    "9" => &8,
    "T" => &9,
    "J" => &10,
    "Q" => &11,
    "K" => &12,
    "A" => &13,
};

static JOKERS_WILD_CARD_MAP: phf::Map<&'static str, &'static i32> = phf_map! {
    "2" => &1,
    "3" => &2,
    "4" => &3,
    "5" => &4,
    "6" => &5,
    "7" => &6,
    "8" => &7,
    "9" => &8,
    "T" => &9,
    "J" => &0,
    "Q" => &11,
    "K" => &12,
    "A" => &13,
};

fn determine_hand_type_jokers_wild(hand: &str) -> HandType {
    let mut card_types: HashMap<char, i32> = HashMap::new();
    let mut joker_count = 0;
    for card in hand.chars() {
        if card != 'J' {
            *card_types.entry(card).or_insert(0) += 1; // tally up each card type in hand
        } else {
            joker_count += 1;
        }
    }

    let mut c: Vec<&i32> = card_types.iter().map(|a| a.1).collect(); // just get the number of occurrences
    c.sort_by(|a, b| b.cmp(a)); // sort by number of occurences

    if joker_count >= 4 {
        // JJJJJ, AJJJJ
        return HandType::FiveOfAKind;
    }

    if c[0] == &5 || c[0] + joker_count == 5 {
        // AAAAA, AAAAJ, AAAJJ, AAJJJ
        return HandType::FiveOfAKind;
    }
    if c[0] == &4 || c[0] + joker_count == 4 {
        // AAAAT, AAAQJ
        return HandType::FourOfAKind;
    }
    if c[0] == &3 {
        // no jokers here otherwise it would already be either 4 or 5 of a kind
        if c[1] == &2 {
            // AAAKK
            return HandType::FullHouse;
        }
        // AAAKQ
        return HandType::ThreeOfAKind;
    }
    if c[0] == &2 {
        if c[1] == &2 {
            if joker_count > 0 {
                //AAKKJ
                return HandType::FullHouse;
            }
            // AAKKQ
            return HandType::TwoPair;
        } else if joker_count > 0 {
            // AAKQJ
            return HandType::ThreeOfAKind;
        }
        // AAKQT
        return HandType::OnePair;
    }

    // no other cards match so it's all on joker strength now...

    if joker_count == 2 {
        // AKQJJ
        return HandType::ThreeOfAKind;
    }
    if joker_count == 1 {
        // AKQTJ
        return HandType::OnePair;
    }
    HandType::HighCard
}

fn determine_hand_type(hand: &str) -> HandType {
    let mut card_types: HashMap<char, i32> = HashMap::new();

    for card in hand.chars() {
        *card_types.entry(card).or_insert(0) += 1; // tally up each card type in hand
    }

    let mut c: Vec<&i32> = card_types.iter().map(|a| a.1).collect(); // just get the number of occurrences
    c.sort_by(|a, b| b.cmp(a)); // sort by number of occurences

    if c[0] == &5 {
        return HandType::FiveOfAKind;
    }
    if c[0] == &4 {
        return HandType::FourOfAKind;
    }
    if c[0] == &3 {
        // no jokers here otherwise it would already be either 4 or 5 of a kind
        if c[1] == &2 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if c[0] == &2 {
        if c[1] == &2 {
            return HandType::TwoPair;
        }
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn part1(lines: &Vec<String>) {
    let mut hands = lines
        .iter()
        .map(|l| {
            // println!("{}, {}", l.trim()[6..].parse::<i32>().unwrap(), l);
            Hand {
                hand_type: determine_hand_type(&l.as_str()[..5]),
                card_vals: CardVal {
                    card_vals: String::from(l.as_str()[..5].to_string())
                        .chars()
                        .map(|c| **CARDS_MAP.get(c.to_string().as_str()).unwrap())
                        .into_iter()
                        .collect::<Vec<i32>>(),
                },
                value: String::from(l.as_str()[..5].to_string()),
                bid: l.trim()[6..].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    let score = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let rank = index as i32 + 1;
        let score = rank * hand.bid;
        // println!(
        //     "hand: {}, type: {}, rank: {}, bid: {}, score: {}, current: {}",
        //     hand.value, hand.hand_type, rank, hand.bid, score, acc
        // );
        acc + score
    });

    // println!("{:?}", hands.iter().map(|h| &h.value));
    println!("PART 1 SCORE: {}", score)
}

fn part2(lines: &Vec<String>) {
    let mut hands = lines
        .iter()
        .map(|l| {
            // println!("{}, {}", l.trim()[6..].parse::<i32>().unwrap(), l);
            Hand {
                hand_type: determine_hand_type_jokers_wild(&l.as_str()[..5]),
                card_vals: CardVal {
                    card_vals: String::from(l.as_str()[..5].to_string())
                        .chars()
                        .map(|c| **JOKERS_WILD_CARD_MAP.get(c.to_string().as_str()).unwrap())
                        .into_iter()
                        .collect::<Vec<i32>>(),
                },
                value: String::from(l.as_str()[..5].to_string()),
                bid: l.trim()[6..].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    let score = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let rank = index as i32 + 1;
        let score = rank * hand.bid;
        // println!(
        //     "hand: {}, type: {}, rank: {}, bid: {}, score: {}, current: {}",
        //     hand.value, hand.hand_type, rank, hand.bid, score, acc
        // );
        acc + score
    });

    // println!("{:?}", hands.iter().map(|h| &h.value));
    println!("PART 2 SCORE: {}", score)
}
