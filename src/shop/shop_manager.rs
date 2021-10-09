use rand::Rng;
use rand::rngs::StdRng;
use crate::data::card::BaseCards;

pub struct ShopManager;

impl ShopManager {
    fn draw_level1(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::Merch1,
            1 => BaseCards::Merch2,
            2 => BaseCards::Mush1,
            3 => BaseCards::Mush2,
            4 => BaseCards::Rob1,
            5 => BaseCards::Rob2,
            6 => BaseCards::Spid1,
            _ => BaseCards::Spid2,
        }
    }

    fn draw_level2(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::Merch3,
            1 => BaseCards::Merch4,
            2 => BaseCards::Mush3,
            3 => BaseCards::Mush4,
            4 => BaseCards::Rob3,
            5 => BaseCards::Rob4,
            6 => BaseCards::Spid3,
            _ => BaseCards::Spid4,
        }
    }

    fn draw_level3(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::Merch5,
            1 => BaseCards::Merch6,
            2 => BaseCards::Mush5,
            3 => BaseCards::Mush6,
            4 => BaseCards::Rob5,
            5 => BaseCards::Rob6,
            6 => BaseCards::Spid5,
            _ => BaseCards::Spid6,
        }
    }

    fn draw_level4(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::Merch7,
            1 => BaseCards::Merch8,
            2 => BaseCards::Mush7,
            3 => BaseCards::Mush8,
            4 => BaseCards::Rob7,
            5 => BaseCards::Rob8,
            6 => BaseCards::Spid7,
            _ => BaseCards::Spid8,
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