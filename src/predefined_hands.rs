use crate::card::{BaseCards, Card};
use crate::GlobalData;

pub struct ChosenHand(pub HandsName);

#[derive(Clone)]
pub enum HandsName {
    Mush,
}

struct CardState(BaseCards, u16, u16);

impl HandsName {

    fn hand_components(&self, turn: u16) -> Vec<CardState> {
        match self {
            HandsName::Mush => match turn {
                1 => vec![
                    CardState(BaseCards::MUSH_2, 1, 1),
                ],
                2 => vec![
                    CardState(BaseCards::MUSH_2, 1, 2),
                ],
                3 => vec![
                    CardState(BaseCards::MUSH_2, 1, 3),
                    CardState(BaseCards::MUSH_2, 1, 1),
                ],
                4 => vec![
                    CardState(BaseCards::MUSH_2, 2, 5),
                    CardState(BaseCards::MUSH_4, 1, 1),
                    CardState(BaseCards::MUSH_2, 2, 3),
                ],
                5 => vec![
                    CardState(BaseCards::MUSH_2, 2, 6),
                    CardState(BaseCards::MUSH_4, 1, 1),
                    CardState(BaseCards::MUSH_3, 2, 5),
                    CardState(BaseCards::MUSH_2, 2, 4),
                ],
                6 => vec![
                    CardState(BaseCards::MUSH_1, 1, 3),
                    CardState(BaseCards::MUSH_2, 2, 7),
                    CardState(BaseCards::MUSH_4, 1, 1),
                    CardState(BaseCards::MUSH_3, 2, 5),
                    CardState(BaseCards::MUSH_2, 2, 5),
                ],
                7 => vec![
                    CardState(BaseCards::MUSH_1, 1, 3),
                    CardState(BaseCards::MUSH_2, 2, 8),
                    CardState(BaseCards::MUSH_4, 1, 1),
                    CardState(BaseCards::MUSH_3, 2, 5),
                    CardState(BaseCards::MUSH_2, 2, 6),
                ],
                8 => vec![
                    CardState(BaseCards::MUSH_1, 1, 3),
                    CardState(BaseCards::MUSH_2, 2, 9),
                    CardState(BaseCards::MUSH_5, 0, 2),
                    CardState(BaseCards::MUSH_4, 1, 1),
                    CardState(BaseCards::MUSH_3, 2, 5),
                    CardState(BaseCards::MUSH_2, 2, 7),
                ],
                9 => vec![
                    CardState(BaseCards::MUSH_1, 2, 4),
                    CardState(BaseCards::MUSH_4, 1, 1),
                    CardState(BaseCards::MUSH_2, 3, 11),
                    CardState(BaseCards::MUSH_5, 1, 3),
                    CardState(BaseCards::MUSH_4, 2, 2),
                    CardState(BaseCards::MUSH_3, 3, 6),
                    CardState(BaseCards::MUSH_2, 3, 9),
                ],
                _ => vec![
                    CardState(BaseCards::MUSH_1, 3, 5),
                    CardState(BaseCards::MUSH_8, 6, 7),
                    CardState(BaseCards::MUSH_2, 4, 13),
                    CardState(BaseCards::MUSH_5, 2, 4),
                    CardState(BaseCards::MUSH_4, 2, 2),
                    CardState(BaseCards::MUSH_3, 4, 7),
                    CardState(BaseCards::MUSH_2, 4, 11),
                ],
            }
        }
    }

    pub fn hand(&self, global_data: &mut GlobalData) -> Vec<Card> {
        let turn = global_data.turn;
        match self {
            HandsName::Mush => self.hand_components(turn)
        }.iter().map(|card_state| {
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