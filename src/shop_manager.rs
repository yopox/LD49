use rand::distributions::{Distribution, Standard};
use rand::Rng;
use rand::rngs::StdRng;
use crate::card::{BaseCards, Card};

pub struct ShopManager;

impl ShopManager {
    fn draw_level1(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::MERCH_1,
            1 => BaseCards::MERCH_2,
            2 => BaseCards::MUSH_1,
            3 => BaseCards::MUSH_2,
            4 => BaseCards::ROB_1,
            5 => BaseCards::ROB_2,
            6 => BaseCards::SPID_1,
            _ => BaseCards::SPID_2,
        }
    }

    fn draw_level2(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::MERCH_3,
            1 => BaseCards::MERCH_4,
            2 => BaseCards::MUSH_3,
            3 => BaseCards::MUSH_4,
            4 => BaseCards::ROB_3,
            5 => BaseCards::ROB_4,
            6 => BaseCards::SPID_3,
            _ => BaseCards::SPID_4,
        }
    }

    fn draw_level3(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::MERCH_5,
            1 => BaseCards::MERCH_6,
            2 => BaseCards::MUSH_5,
            3 => BaseCards::MUSH_6,
            4 => BaseCards::ROB_5,
            5 => BaseCards::ROB_6,
            6 => BaseCards::SPID_5,
            _ => BaseCards::SPID_6,
        }
    }

    fn draw_level4(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::MERCH_7,
            1 => BaseCards::MERCH_8,
            2 => BaseCards::MUSH_7,
            3 => BaseCards::MUSH_8,
            4 => BaseCards::ROB_7,
            5 => BaseCards::ROB_8,
            6 => BaseCards::SPID_7,
            _ => BaseCards::SPID_8,
        }
    }

    fn cards_amount(level: u16) -> u16 {
        match level {
            1 => 3,
            2 => 4,
            3 => 5,
            _ => 6,
        }
    }

    pub fn shop_inventory(level: u16, rng: &mut StdRng) -> Vec<BaseCards> {
        let mut cards = vec![];
        for _ in 0..ShopManager::cards_amount(level) {
            match level {
                1 => cards.push(ShopManager::draw_level1(rng)),
                2 => match rng.gen_range(0..2) {
                    0 => cards.push(ShopManager::draw_level1(rng)),
                    _ => cards.push(ShopManager::draw_level2(rng)),
                }
                3 => match rng.gen_range(0..5) {
                    0 => cards.push(ShopManager::draw_level1(rng)),
                    1|2 => cards.push(ShopManager::draw_level2(rng)),
                    _ => cards.push(ShopManager::draw_level3(rng)),
                }
                _ => match rng.gen_range(0..11) {
                    0|1 => cards.push(ShopManager::draw_level1(rng)),
                    2|3|4 => cards.push(ShopManager::draw_level2(rng)),
                    5|6|7|8 => cards.push(ShopManager::draw_level3(rng)),
                    _ => cards.push(ShopManager::draw_level4(rng)),
                }
            }
        }
        return cards;
    }

}