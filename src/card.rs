use serde_derive::*;
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card {
    number: usize,
    color: Color,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.number != other.number {
            self.number.cmp(&other.number)
        } else {
            self.color.cmp(&other.color)
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Red,
    Blue,
    Yellow,
}

impl Ord for Color {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Color::Red, Color::Blue) => {
                std::cmp::Ordering::Less
            },
            (Color::Blue, Color::Red) => {
                std::cmp::Ordering::Greater
            },
            (_, _) => {
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// make cards
pub fn make_cards() -> Vec<Card> {
    let data = [
        (Color::Red, 0),
        (Color::Red, 1),
        (Color::Red, 2),
        (Color::Red, 3),
        (Color::Red, 4),
        (Color::Red, 6),
        (Color::Red, 7),
        (Color::Red, 8),
        (Color::Red, 9),
        (Color::Blue, 0),
        (Color::Blue, 1),
        (Color::Blue, 2),
        (Color::Blue, 3),
        (Color::Blue, 4),
        (Color::Blue, 6),
        (Color::Blue, 7),
        (Color::Blue, 8),
        (Color::Blue, 9),
        (Color::Yellow, 5),
        (Color::Yellow, 5),
    ];

    data.iter()
        .map(|&(color, number)| Card { color, number })
        .collect::<Vec<Card>>()
}
