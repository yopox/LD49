use rand::distributions::{Distribution, Standard};
use rand::Rng;
use crate::data::card::{BaseCards, Card};
use crate::GlobalData;

#[derive(Clone)]
pub enum HandsName {
    Mush,
    Spiders,
}

impl Distribution<HandsName> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HandsName {
        match rng.gen_range(0..=1) {
            0 => HandsName::Mush,
            1 => HandsName::Spiders,
            _ => HandsName::Mush,
        }
    }
}

struct CardState(BaseCards, u16, u16);

impl HandsName {
    fn hand_components(&self, turn: u16) -> Vec<CardState> {
        match self {
            HandsName::Mush => match turn {
                1 => vec![
                    CardState(BaseCards::Mush2, 1, 1),
                ],
                2 => vec![
                    CardState(BaseCards::Mush2, 1, 2),
                ],
                3 => vec![
                    CardState(BaseCards::Mush2, 1, 3),
                    CardState(BaseCards::Mush2, 1, 1),
                ],
                4 => vec![
                    CardState(BaseCards::Mush2, 2, 5),
                    CardState(BaseCards::Mush4, 1, 1),
                    CardState(BaseCards::Mush2, 2, 3),
                ],
                5 => vec![
                    CardState(BaseCards::Mush2, 2, 6),
                    CardState(BaseCards::Mush4, 1, 1),
                    CardState(BaseCards::Mush3, 2, 5),
                    CardState(BaseCards::Mush2, 2, 4),
                ],
                6 => vec![
                    CardState(BaseCards::Mush1, 1, 3),
                    CardState(BaseCards::Mush2, 2, 7),
                    CardState(BaseCards::Mush4, 1, 1),
                    CardState(BaseCards::Mush3, 2, 5),
                    CardState(BaseCards::Mush2, 2, 5),
                ],
                7 => vec![
                    CardState(BaseCards::Mush1, 1, 3),
                    CardState(BaseCards::Mush2, 2, 8),
                    CardState(BaseCards::Mush4, 1, 1),
                    CardState(BaseCards::Mush3, 2, 5),
                    CardState(BaseCards::Mush2, 2, 6),
                ],
                8 => vec![
                    CardState(BaseCards::Mush1, 1, 3),
                    CardState(BaseCards::Mush2, 2, 9),
                    CardState(BaseCards::Mush5, 0, 2),
                    CardState(BaseCards::Mush4, 1, 1),
                    CardState(BaseCards::Mush3, 2, 5),
                    CardState(BaseCards::Mush2, 2, 7),
                ],
                9 => vec![
                    CardState(BaseCards::Mush1, 2, 4),
                    CardState(BaseCards::Mush4, 1, 1),
                    CardState(BaseCards::Mush2, 3, 11),
                    CardState(BaseCards::Mush5, 1, 3),
                    CardState(BaseCards::Mush4, 2, 2),
                    CardState(BaseCards::Mush3, 3, 6),
                    CardState(BaseCards::Mush2, 3, 9),
                ],
                _ => vec![
                    CardState(BaseCards::Mush1, 3, 5),
                    CardState(BaseCards::Mush8, 6, 7),
                    CardState(BaseCards::Mush2, 4, 13),
                    CardState(BaseCards::Mush5, 2, 4),
                    CardState(BaseCards::Mush4, 2, 2),
                    CardState(BaseCards::Mush3, 4, 7),
                    CardState(BaseCards::Mush2, 4, 11),
                ],
            }
            HandsName::Spiders => match turn {
                1 => vec![
                    CardState(BaseCards::Spid2, 2, 2),
                ],
                2 => vec![
                    CardState(BaseCards::Spid2, 2, 2),
                    CardState(BaseCards::Spid1, 2, 2),
                ],
                3 => vec![
                    CardState(BaseCards::Spid1, 2, 1),
                    CardState(BaseCards::Spid2, 2, 2),
                ],
                4 => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid2, 2, 2),
                    CardState(BaseCards::Spid1, 2, 1),
                ],
                5 => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid4, 3, 3),
                    CardState(BaseCards::Spid2, 2, 2),
                    CardState(BaseCards::Spid1, 2, 1),
                ],
                6 => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid4, 3, 3),
                    CardState(BaseCards::Spid2, 2, 2),
                    CardState(BaseCards::Spid1, 2, 1),
                ],
                7 => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid4, 3, 3),
                    CardState(BaseCards::Spid2, 2, 2),
                    CardState(BaseCards::Spid1, 2, 1),
                ],
                8 => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid4, 3, 3),
                    CardState(BaseCards::Spid2, 2, 2),
                    CardState(BaseCards::Spid1, 2, 1),
                ],
                9 => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid4, 3, 3),
                    CardState(BaseCards::Spid2, 2, 2),
                    CardState(BaseCards::Spid1, 2, 1),
                ],
                10 => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid7, 6, 4),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid4, 3, 3),
                ],
                _ => vec![
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid3, 3, 2),
                    CardState(BaseCards::Spid7, 6, 4),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid6, 5, 2),
                    CardState(BaseCards::Spid4, 3, 3),
                    CardState(BaseCards::Spid1, 2, 7),
                ],
            }
        }
    }

    pub fn hand(&self, global_data: &mut GlobalData) -> Vec<Card> {
        let turn = global_data.turn;
        self.hand_components(turn).iter().map(|card_state| {
            let card = Card {
                base_card: card_state.0,
                id: global_data.next_card_id,
                hp: card_state.2,
                atk: card_state.1,
                played: 0,
            };
            global_data.next_card_id += 1;
            card
        }).collect()
    }
}