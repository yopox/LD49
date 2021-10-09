use bevy::prelude::*;
use derive_more::Display;
use rand::Rng;
use rand::rngs::StdRng;

use crate::data::loading::TextureAssets;

pub const CARD_SCALE: f32 = 0.4;
pub const CARD_WIDTH: f32 = 270. * CARD_SCALE;
pub const CARD_HEIGHT: f32 = 420. * CARD_SCALE;

#[derive(Copy, Clone)]
pub enum BaseCards {
    Mush1,
    Mush2,
    Mush3,
    Mush4,
    Mush5,
    Mush6,
    Mush7,
    Mush8,

    Merch1,
    Merch2,
    Merch3,
    Merch4,
    Merch5,
    Merch6,
    Merch7,
    Merch8,

    Spid1,
    Spid2,
    Spid3,
    Spid4,
    Spid5,
    Spid6,
    Spid7,
    Spid8,

    Rob1,
    Rob2,
    Rob3,
    Rob4,
    Rob5,
    Rob6,
    Rob7,
    Rob8,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub base_card: BaseCards,
    pub id: u32,
    pub hp: u16,
    pub atk: u16,
    pub played: u8,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            base_card: BaseCards::Merch8,
            id: 0,
            hp: 0,
            atk: 0,
            played: 0,
        }
    }
}

#[derive(Debug, Display, PartialEq, Eq, Copy, Clone)]
pub enum Abilities {
    // Mushrooms
    Slimy,
    SweetScent,
    ToxicSpores,
    Sporocarp,
    Roots,
    Gigantism,
    // Merchants
    Sadism,
    ExplodingArmour,
    Pillage,
    GoldMine,
    Altruism,
    Dexterity,
    // Spiders
    Cooperation,
    Trap,
    Multiplication,
    Poisonous,
    Spawn,
    Cannibalism,
    // Robots
    Replication,
    Scanner,
    Upgrade,
    Glitch,
    Upload,
    Download,

    // None or not at combat time
    None,
}

#[derive(Debug, Display, PartialEq, Eq, Copy, Clone)]
pub enum Triggers {
    // At the beginning of each turn
    Turn,
    // When this card is played
    Played,
    // When this card dies
    Death,
    // When this card attacks and survives
    Survived,
    // When this card attacks or is attacked
    Hit,
    // When this card kills
    Kill,
    // When this card is sold
    Sold,
    // PASSIVE,
    None,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Families {
    Merchants,
    Spiders,
    Robots,
    Mushrooms,
}

impl BaseCards {
    pub fn name(&self) -> &'static str {
        match self {
            BaseCards::Mush1 => "Coprinus",
            BaseCards::Mush2 => "Gomphidius",
            BaseCards::Mush3 => "Cantharellus",
            BaseCards::Mush4 => "Mycelius",
            BaseCards::Mush5 => "Amanita",
            BaseCards::Mush6 => "Boletus",
            BaseCards::Mush7 => "Silex fungi",
            BaseCards::Mush8 => "Titanicus",
            BaseCards::Merch1 => "Silvanus",
            BaseCards::Merch2 => "Estan",
            BaseCards::Merch3 => "Mandra",
            BaseCards::Merch4 => "Julius",
            BaseCards::Merch5 => "Sixante",
            BaseCards::Merch6 => "Eladra",
            BaseCards::Merch7 => "Jasmar",
            BaseCards::Merch8 => "Tujilus",
            BaseCards::Spid1 => "Micro spider",
            BaseCards::Spid2 => "Jumping Spider",
            BaseCards::Spid3 => "Funnel Web Spider",
            BaseCards::Spid4 => "Goliath",
            BaseCards::Spid5 => "Egg",
            BaseCards::Spid6 => "Tarantula",
            BaseCards::Spid7 => "Aragog",
            BaseCards::Spid8 => "Australian black widow",
            BaseCards::Rob1 => "Nanobot",
            BaseCards::Rob2 => "Cleaning robot",
            BaseCards::Rob3 => "Turret",
            BaseCards::Rob4 => "Modular bot",
            BaseCards::Rob5 => "Unfinished prototype",
            BaseCards::Rob6 => "Humanoid",
            BaseCards::Rob7 => "Repair drones",
            BaseCards::Rob8 => "SkyBot",
        }
    }

    pub fn ability(&self) -> Abilities {
        match self {
            BaseCards::Mush1 => Abilities::None,
            BaseCards::Mush2 => Abilities::Slimy,
            BaseCards::Mush3 => Abilities::None,
            BaseCards::Mush4 => Abilities::SweetScent,
            BaseCards::Mush5 => Abilities::ToxicSpores,
            BaseCards::Mush6 => Abilities::Sporocarp,
            BaseCards::Mush7 => Abilities::Roots,
            BaseCards::Mush8 => Abilities::Gigantism,
            BaseCards::Merch1 => Abilities::None,
            BaseCards::Merch2 => Abilities::Sadism,
            BaseCards::Merch3 => Abilities::ExplodingArmour,
            BaseCards::Merch4 => Abilities::None,
            BaseCards::Merch5 => Abilities::Pillage,
            BaseCards::Merch6 => Abilities::GoldMine,
            BaseCards::Merch7 => Abilities::Altruism,
            BaseCards::Merch8 => Abilities::Dexterity,
            BaseCards::Spid1 => Abilities::Cooperation,
            BaseCards::Spid2 => Abilities::None,
            BaseCards::Spid3 => Abilities::Trap,
            BaseCards::Spid4 => Abilities::None,
            BaseCards::Spid5 => Abilities::Multiplication,
            BaseCards::Spid6 => Abilities::Poisonous,
            BaseCards::Spid7 => Abilities::Spawn,
            BaseCards::Spid8 => Abilities::Cannibalism,
            BaseCards::Rob1 => Abilities::Replication,
            BaseCards::Rob2 => Abilities::None,
            BaseCards::Rob3 => Abilities::Scanner,
            BaseCards::Rob4 => Abilities::Upgrade,
            BaseCards::Rob5 => Abilities::Glitch,
            BaseCards::Rob6 => Abilities::None,
            BaseCards::Rob7 => Abilities::Upload,
            BaseCards::Rob8 => Abilities::Download,
        }
    }

    pub fn family(&self) -> Families {
        match self {
            BaseCards::Mush1 => Families::Mushrooms,
            BaseCards::Mush2 => Families::Mushrooms,
            BaseCards::Mush3 => Families::Mushrooms,
            BaseCards::Mush4 => Families::Mushrooms,
            BaseCards::Mush5 => Families::Mushrooms,
            BaseCards::Mush6 => Families::Mushrooms,
            BaseCards::Mush7 => Families::Mushrooms,
            BaseCards::Mush8 => Families::Mushrooms,
            BaseCards::Merch1 => Families::Merchants,
            BaseCards::Merch2 => Families::Merchants,
            BaseCards::Merch3 => Families::Merchants,
            BaseCards::Merch4 => Families::Merchants,
            BaseCards::Merch5 => Families::Merchants,
            BaseCards::Merch6 => Families::Merchants,
            BaseCards::Merch7 => Families::Merchants,
            BaseCards::Merch8 => Families::Merchants,
            BaseCards::Spid1 => Families::Spiders,
            BaseCards::Spid2 => Families::Spiders,
            BaseCards::Spid3 => Families::Spiders,
            BaseCards::Spid4 => Families::Spiders,
            BaseCards::Spid5 => Families::Spiders,
            BaseCards::Spid6 => Families::Spiders,
            BaseCards::Spid7 => Families::Spiders,
            BaseCards::Spid8 => Families::Spiders,
            BaseCards::Rob1 => Families::Robots,
            BaseCards::Rob2 => Families::Robots,
            BaseCards::Rob3 => Families::Robots,
            BaseCards::Rob4 => Families::Robots,
            BaseCards::Rob5 => Families::Robots,
            BaseCards::Rob6 => Families::Robots,
            BaseCards::Rob7 => Families::Robots,
            BaseCards::Rob8 => Families::Robots,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BaseCards::Mush1 => "",
            BaseCards::Mush2 => "This card gets +1 HP.",
            BaseCards::Mush3 => "",
            BaseCards::Mush4 => "Gives +1 HP & +1 ATK\nto other Mush cards.",
            BaseCards::Mush5 => "Toxic spores are released\nand kill the opponent.",
            BaseCards::Mush6 => "Adds a random Mush\nto your hand.",
            BaseCards::Mush7 => "Gets +1 HP for each allied Mush.",
            BaseCards::Mush8 => "Gets +1 ATK.",
            BaseCards::Merch1 => "",
            BaseCards::Merch2 => "Attack a random ally\nonce (50%).",
            BaseCards::Merch3 => "Deals 1 DMG to every\nenemy card.",
            BaseCards::Merch4 => "",
            BaseCards::Merch5 => "Gain +1 gold for the\nnext turn.",
            BaseCards::Merch6 => "Gain +1 gold for this turn.",
            BaseCards::Merch7 => "Distribute its max HP\nbetween allies.",
            BaseCards::Merch8 => "Attacks another enemy once.",
            BaseCards::Spid1 => "Gets +1 HP for each spider on board.",
            BaseCards::Spid2 => "",
            BaseCards::Spid3 => "Lowers the opponents\nattack by half.",
            BaseCards::Spid4 => "",
            BaseCards::Spid5 => "Fills the board with\nlower rank spiders.",
            BaseCards::Spid6 => "Bites the opponent\nand kills it.",
            BaseCards::Spid7 => "Spawns a new Rank 1 Spider.",
            BaseCards::Spid8 => "Eats the lowest rank\nspider of the board,\nand gains its stats.",
            BaseCards::Rob1 => "Spawns a new Nanobot.",
            BaseCards::Rob2 => "",
            BaseCards::Rob3 => "Gains +1 HP and +1 ATK\nfor each allied Nanobot.",
            BaseCards::Rob4 => "Gives +1 HP and +1 ATK\nto itself and adjacent cards.",
            BaseCards::Rob5 => "Removes 2 HP max or\n2 ATK max to the enemy.",
            BaseCards::Rob6 => "",
            BaseCards::Rob7 => "This loses 2 HP\nand gives +2 HP\nto allied robots.",
            BaseCards::Rob8 => "Steals +1 HP and +1 ATK\nfrom each allied robot.",
        }
    }

    pub fn rank(&self) -> u8 {
        match self {
            BaseCards::Mush1 => 1,
            BaseCards::Mush2 => 1,
            BaseCards::Mush3 => 2,
            BaseCards::Mush4 => 2,
            BaseCards::Mush5 => 3,
            BaseCards::Mush6 => 3,
            BaseCards::Mush7 => 4,
            BaseCards::Mush8 => 4,
            BaseCards::Merch1 => 1,
            BaseCards::Merch2 => 1,
            BaseCards::Merch3 => 2,
            BaseCards::Merch4 => 2,
            BaseCards::Merch5 => 3,
            BaseCards::Merch6 => 3,
            BaseCards::Merch7 => 4,
            BaseCards::Merch8 => 4,
            BaseCards::Spid1 => 1,
            BaseCards::Spid2 => 1,
            BaseCards::Spid3 => 2,
            BaseCards::Spid4 => 2,
            BaseCards::Spid5 => 3,
            BaseCards::Spid6 => 3,
            BaseCards::Spid7 => 4,
            BaseCards::Spid8 => 4,
            BaseCards::Rob1 => 1,
            BaseCards::Rob2 => 1,
            BaseCards::Rob3 => 2,
            BaseCards::Rob4 => 2,
            BaseCards::Rob5 => 3,
            BaseCards::Rob6 => 3,
            BaseCards::Rob7 => 4,
            BaseCards::Rob8 => 4,
        }
    }

    pub fn trigger(&self) -> Triggers {
        match self {
            BaseCards::Mush1 => Triggers::None,
            BaseCards::Mush2 => Triggers::Turn,
            BaseCards::Mush3 => Triggers::None,
            BaseCards::Mush4 => Triggers::Played,
            BaseCards::Mush5 => Triggers::Death,
            BaseCards::Mush6 => Triggers::Sold,
            BaseCards::Mush7 => Triggers::Turn,
            BaseCards::Mush8 => Triggers::Kill,
            BaseCards::Merch1 => Triggers::None,
            BaseCards::Merch2 => Triggers::Survived,
            BaseCards::Merch3 => Triggers::Death,
            BaseCards::Merch4 => Triggers::None,
            BaseCards::Merch5 => Triggers::Kill,
            BaseCards::Merch6 => Triggers::Turn,
            BaseCards::Merch7 => Triggers::Sold,
            BaseCards::Merch8 => Triggers::Survived,
            BaseCards::Spid1 => Triggers::Played,
            BaseCards::Spid2 => Triggers::None,
            BaseCards::Spid3 => Triggers::Hit,
            BaseCards::Spid4 => Triggers::None,
            BaseCards::Spid5 => Triggers::Death,
            BaseCards::Spid6 => Triggers::Hit,
            BaseCards::Spid7 => Triggers::Turn,
            BaseCards::Spid8 => Triggers::Turn,
            BaseCards::Rob1 => Triggers::Played,
            BaseCards::Rob2 => Triggers::None,
            BaseCards::Rob3 => Triggers::Played,
            BaseCards::Rob4 => Triggers::Turn,
            BaseCards::Rob5 => Triggers::Hit,
            BaseCards::Rob6 => Triggers::None,
            BaseCards::Rob7 => Triggers::Turn,
            BaseCards::Rob8 => Triggers::Turn,
        }
    }

    pub fn handle(&self, handles: &Res<TextureAssets>) -> Handle<ColorMaterial> {
        match self {
            BaseCards::Mush1 => handles.mush_1.clone(),
            BaseCards::Mush2 => handles.mush_2.clone(),
            BaseCards::Mush3 => handles.mush_3.clone(),
            BaseCards::Mush4 => handles.mush_4.clone(),
            BaseCards::Mush5 => handles.mush_5.clone(),
            BaseCards::Mush6 => handles.mush_6.clone(),
            BaseCards::Mush7 => handles.mush_7.clone(),
            BaseCards::Mush8 => handles.mush_8.clone(),
            BaseCards::Merch1 => handles.merch_1.clone(),
            BaseCards::Merch2 => handles.merch_2.clone(),
            BaseCards::Merch3 => handles.merch_3.clone(),
            BaseCards::Merch4 => handles.merch_4.clone(),
            BaseCards::Merch5 => handles.merch_5.clone(),
            BaseCards::Merch6 => handles.merch_6.clone(),
            BaseCards::Merch7 => handles.merch_7.clone(),
            BaseCards::Merch8 => handles.merch_8.clone(),
            BaseCards::Spid1 => handles.spid_1.clone(),
            BaseCards::Spid2 => handles.spid_2.clone(),
            BaseCards::Spid3 => handles.spid_3.clone(),
            BaseCards::Spid4 => handles.spid_4.clone(),
            BaseCards::Spid5 => handles.spid_5.clone(),
            BaseCards::Spid6 => handles.spid_6.clone(),
            BaseCards::Spid7 => handles.spid_7.clone(),
            BaseCards::Spid8 => handles.spid_8.clone(),
            BaseCards::Rob1 => handles.rob_1.clone(),
            BaseCards::Rob2 => handles.rob_2.clone(),
            BaseCards::Rob3 => handles.rob_3.clone(),
            BaseCards::Rob4 => handles.rob_4.clone(),
            BaseCards::Rob5 => handles.rob_5.clone(),
            BaseCards::Rob6 => handles.rob_6.clone(),
            BaseCards::Rob7 => handles.rob_7.clone(),
            BaseCards::Rob8 => handles.rob_8.clone(),
        }
    }

    pub fn random_mush(rng: &mut StdRng) -> BaseCards {
        match rng.gen_range(0..8) {
            0 => BaseCards::Mush1,
            1 => BaseCards::Mush2,
            2 => BaseCards::Mush3,
            3 => BaseCards::Mush4,
            4 => BaseCards::Mush5,
            5 => BaseCards::Mush6,
            6 => BaseCards::Mush7,
            _ => BaseCards::Mush8,
        }
    }
}

impl Card {
    pub(crate) fn new(card_type: BaseCards, id: u32) -> Self {
        match card_type {
            BaseCards::Mush1 => Card { id, base_card: card_type, atk: 1, hp: 3, ..Default::default() },
            BaseCards::Mush2 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::Mush3 => Card { id, base_card: card_type, atk: 2, hp: 5, ..Default::default() },
            BaseCards::Mush4 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::Mush5 => Card { id, base_card: card_type, atk: 0, hp: 2, ..Default::default() },
            BaseCards::Mush6 => Card { id, base_card: card_type, atk: 4, hp: 2, ..Default::default() },
            BaseCards::Mush7 => Card { id, base_card: card_type, atk: 6, hp: 2, ..Default::default() },
            BaseCards::Mush8 => Card { id, base_card: card_type, atk: 5, hp: 6, ..Default::default() },
            BaseCards::Merch1 => Card { id, base_card: card_type, atk: 1, hp: 3, ..Default::default() },
            BaseCards::Merch2 => Card { id, base_card: card_type, atk: 1, hp: 4, ..Default::default() },
            BaseCards::Merch3 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::Merch4 => Card { id, base_card: card_type, atk: 2, hp: 5, ..Default::default() },
            BaseCards::Merch5 => Card { id, base_card: card_type, atk: 5, hp: 3, ..Default::default() },
            BaseCards::Merch6 => Card { id, base_card: card_type, atk: 1, hp: 7, ..Default::default() },
            BaseCards::Merch7 => Card { id, base_card: card_type, atk: 4, hp: 8, ..Default::default() },
            BaseCards::Merch8 => Card { id, base_card: card_type, atk: 5, hp: 9, ..Default::default() },
            BaseCards::Spid1 => Card { id, base_card: card_type, atk: 2, hp: 1, ..Default::default() },
            BaseCards::Spid2 => Card { id, base_card: card_type, atk: 2, hp: 2, ..Default::default() },
            BaseCards::Spid3 => Card { id, base_card: card_type, atk: 3, hp: 2, ..Default::default() },
            BaseCards::Spid4 => Card { id, base_card: card_type, atk: 3, hp: 3, ..Default::default() },
            BaseCards::Spid5 => Card { id, base_card: card_type, atk: 0, hp: 1, ..Default::default() },
            BaseCards::Spid6 => Card { id, base_card: card_type, atk: 5, hp: 2, ..Default::default() },
            BaseCards::Spid7 => Card { id, base_card: card_type, atk: 4, hp: 6, ..Default::default() },
            BaseCards::Spid8 => Card { id, base_card: card_type, atk: 5, hp: 6, ..Default::default() },
            BaseCards::Rob1 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::Rob2 => Card { id, base_card: card_type, atk: 1, hp: 3, ..Default::default() },
            BaseCards::Rob3 => Card { id, base_card: card_type, atk: 3, hp: 1, ..Default::default() },
            BaseCards::Rob4 => Card { id, base_card: card_type, atk: 2, hp: 2, ..Default::default() },
            BaseCards::Rob5 => Card { id, base_card: card_type, atk: 4, hp: 2, ..Default::default() },
            BaseCards::Rob6 => Card { id, base_card: card_type, atk: 4, hp: 8, ..Default::default() },
            BaseCards::Rob7 => Card { id, base_card: card_type, atk: 5, hp: 12, ..Default::default() },
            BaseCards::Rob8 => Card { id, base_card: card_type, atk: 3, hp: 3, ..Default::default() },
        }
    }
}