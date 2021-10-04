use std::collections::HashMap;
use std::error::Error;

use bevy::ecs::system::EntityCommands;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use derive_more::Display;

use crate::{AppState, HEIGHT, MainCamera, WIDTH};
use crate::font::*;
use crate::loading::{ColorAssets, TextureAssets};
use crate::ui::Dragged;
use crate::util::{cursor_pos, overlap, Z_POPUP_BG, Z_POPUP_TEXT, Z_STATS};

pub const CARD_SCALE: f32 = 0.4;
pub const CARD_WIDTH: f32 = 270. * CARD_SCALE;
pub const CARD_HEIGHT: f32 = 420. * CARD_SCALE;

#[derive(Copy, Clone)]
pub enum BaseCards {
    MUSH_1,
    MUSH_2,
    MUSH_3,
    MUSH_4,
    MUSH_5,
    MUSH_6,
    MUSH_7,
    MUSH_8,

    MERCH_1,
    MERCH_2,
    MERCH_3,
    MERCH_4,
    MERCH_5,
    MERCH_6,
    MERCH_7,
    MERCH_8,

    SPID_1,
    SPID_2,
    SPID_3,
    SPID_4,
    SPID_5,
    SPID_6,
    SPID_7,
    SPID_8,

    ROB_1,
    ROB_2,
    ROB_3,
    ROB_4,
    ROB_5,
    ROB_6,
    ROB_7,
    ROB_8,
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
            base_card: BaseCards::MERCH_8,
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
            BaseCards::MUSH_1 => "Coprinus",
            BaseCards::MUSH_2 => "Gomphidius",
            BaseCards::MUSH_3 => "Cantharellus",
            BaseCards::MUSH_4 => "Mycelius",
            BaseCards::MUSH_5 => "Amanita",
            BaseCards::MUSH_6 => "Boletus",
            BaseCards::MUSH_7 => "Silex fungi",
            BaseCards::MUSH_8 => "Titanicus",
            BaseCards::MERCH_1 => "Silvanus",
            BaseCards::MERCH_2 => "Estan",
            BaseCards::MERCH_3 => "Mandra",
            BaseCards::MERCH_4 => "Julius",
            BaseCards::MERCH_5 => "Sixante",
            BaseCards::MERCH_6 => "Eladra",
            BaseCards::MERCH_7 => "Jasmar",
            BaseCards::MERCH_8 => "Tujilus",
            BaseCards::SPID_1 => "Micro spider",
            BaseCards::SPID_2 => "Jumping Spider",
            BaseCards::SPID_3 => "Funnel Web Spider",
            BaseCards::SPID_4 => "Goliath",
            BaseCards::SPID_5 => "Egg",
            BaseCards::SPID_6 => "Tarantula",
            BaseCards::SPID_7 => "Aragog",
            BaseCards::SPID_8 => "Australian black widow",
            BaseCards::ROB_1 => "Nanobot",
            BaseCards::ROB_2 => "Cleaning robot",
            BaseCards::ROB_3 => "Turret",
            BaseCards::ROB_4 => "Modular bot",
            BaseCards::ROB_5 => "Unfinished prototype",
            BaseCards::ROB_6 => "Humanoid",
            BaseCards::ROB_7 => "Repair drones",
            BaseCards::ROB_8 => "SkyBot",
        }
    }

    pub fn ability(&self) -> Abilities {
        match self {
            BaseCards::MUSH_1 => Abilities::None,
            BaseCards::MUSH_2 => Abilities::Slimy,
            BaseCards::MUSH_3 => Abilities::None,
            BaseCards::MUSH_4 => Abilities::SweetScent,
            BaseCards::MUSH_5 => Abilities::ToxicSpores,
            BaseCards::MUSH_6 => Abilities::Sporocarp,
            BaseCards::MUSH_7 => Abilities::Roots,
            BaseCards::MUSH_8 => Abilities::Gigantism,
            BaseCards::MERCH_1 => Abilities::None,
            BaseCards::MERCH_2 => Abilities::Sadism,
            BaseCards::MERCH_3 => Abilities::ExplodingArmour,
            BaseCards::MERCH_4 => Abilities::None,
            BaseCards::MERCH_5 => Abilities::Pillage,
            BaseCards::MERCH_6 => Abilities::GoldMine,
            BaseCards::MERCH_7 => Abilities::Altruism,
            BaseCards::MERCH_8 => Abilities::Dexterity,
            BaseCards::SPID_1 => Abilities::Cooperation,
            BaseCards::SPID_2 => Abilities::None,
            BaseCards::SPID_3 => Abilities::Trap,
            BaseCards::SPID_4 => Abilities::None,
            BaseCards::SPID_5 => Abilities::Multiplication,
            BaseCards::SPID_6 => Abilities::Poisonous,
            BaseCards::SPID_7 => Abilities::Spawn,
            BaseCards::SPID_8 => Abilities::Cannibalism,
            BaseCards::ROB_1 => Abilities::Replication,
            BaseCards::ROB_2 => Abilities::None,
            BaseCards::ROB_3 => Abilities::Scanner,
            BaseCards::ROB_4 => Abilities::Upgrade,
            BaseCards::ROB_5 => Abilities::Glitch,
            BaseCards::ROB_6 => Abilities::None,
            BaseCards::ROB_7 => Abilities::Upload,
            BaseCards::ROB_8 => Abilities::Download,
        }
    }

    pub fn family(&self) -> Families {
        match self {
            BaseCards::MUSH_1 => Families::Mushrooms,
            BaseCards::MUSH_2 => Families::Mushrooms,
            BaseCards::MUSH_3 => Families::Mushrooms,
            BaseCards::MUSH_4 => Families::Mushrooms,
            BaseCards::MUSH_5 => Families::Mushrooms,
            BaseCards::MUSH_6 => Families::Mushrooms,
            BaseCards::MUSH_7 => Families::Mushrooms,
            BaseCards::MUSH_8 => Families::Mushrooms,
            BaseCards::MERCH_1 => Families::Merchants,
            BaseCards::MERCH_2 => Families::Merchants,
            BaseCards::MERCH_3 => Families::Merchants,
            BaseCards::MERCH_4 => Families::Merchants,
            BaseCards::MERCH_5 => Families::Merchants,
            BaseCards::MERCH_6 => Families::Merchants,
            BaseCards::MERCH_7 => Families::Merchants,
            BaseCards::MERCH_8 => Families::Merchants,
            BaseCards::SPID_1 => Families::Spiders,
            BaseCards::SPID_2 => Families::Spiders,
            BaseCards::SPID_3 => Families::Spiders,
            BaseCards::SPID_4 => Families::Spiders,
            BaseCards::SPID_5 => Families::Spiders,
            BaseCards::SPID_6 => Families::Spiders,
            BaseCards::SPID_7 => Families::Spiders,
            BaseCards::SPID_8 => Families::Spiders,
            BaseCards::ROB_1 => Families::Robots,
            BaseCards::ROB_2 => Families::Robots,
            BaseCards::ROB_3 => Families::Robots,
            BaseCards::ROB_4 => Families::Robots,
            BaseCards::ROB_5 => Families::Robots,
            BaseCards::ROB_6 => Families::Robots,
            BaseCards::ROB_7 => Families::Robots,
            BaseCards::ROB_8 => Families::Robots,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BaseCards::MUSH_1 => "",
            BaseCards::MUSH_2 => "This card gets +1 HP.",
            BaseCards::MUSH_3 => "",
            BaseCards::MUSH_4 => "Gives +1 HP & +1 ATK\nto other Mush cards.",
            BaseCards::MUSH_5 => "Toxic spores are released\nand kill the opponent.",
            BaseCards::MUSH_6 => "Adds a random Mush\nto your hand.",
            BaseCards::MUSH_7 => "Gets +1 HP for each allied Mush.",
            BaseCards::MUSH_8 => "Gets +1 ATK.",
            BaseCards::MERCH_1 => "",
            BaseCards::MERCH_2 => "Attack a random ally\nonce (50%).",
            BaseCards::MERCH_3 => "Deals 1 DMG to every\nenemy card.",
            BaseCards::MERCH_4 => "",
            BaseCards::MERCH_5 => "Gain +1 gold for the\nnext turn.",
            BaseCards::MERCH_6 => "Gain +1 gold for this turn.",
            BaseCards::MERCH_7 => "Distribute its max HP\nbetween allies.",
            BaseCards::MERCH_8 => "Attacks another enemy once.",
            BaseCards::SPID_1 => "Gets +1 HP for each spider on board.",
            BaseCards::SPID_2 => "",
            BaseCards::SPID_3 => "Lowers the opponents\nattack by half.",
            BaseCards::SPID_4 => "",
            BaseCards::SPID_5 => "Fills the board with\nlower rank spiders.",
            BaseCards::SPID_6 => "Bites the opponent\nand kills it.",
            BaseCards::SPID_7 => "Spawns a new Rank 1 Spider.",
            BaseCards::SPID_8 => "Eats the lowest rank\nspider of the board,\nand gains its stats.",
            BaseCards::ROB_1 => "Spawns a new Nanobot.",
            BaseCards::ROB_2 => "",
            BaseCards::ROB_3 => "Gains +1 HP and +1 ATK\nfor each allied Nanobot.",
            BaseCards::ROB_4 => "Gives +1 HP and +1 ATK\nto itself and adjacent cards.",
            BaseCards::ROB_5 => "Removes 2 HP max or\n2 ATK max to the enemy.",
            BaseCards::ROB_6 => "",
            BaseCards::ROB_7 => "This loses 2 HP\nand gives +2 HP\nto allied robots.",
            BaseCards::ROB_8 => "Steals +1 HP and +1 ATK\nfrom each allied robot.",
        }
    }

    pub fn rank(&self) -> u8 {
        match self {
            BaseCards::MUSH_1 => 1,
            BaseCards::MUSH_2 => 1,
            BaseCards::MUSH_3 => 2,
            BaseCards::MUSH_4 => 2,
            BaseCards::MUSH_5 => 3,
            BaseCards::MUSH_6 => 3,
            BaseCards::MUSH_7 => 4,
            BaseCards::MUSH_8 => 4,
            BaseCards::MERCH_1 => 1,
            BaseCards::MERCH_2 => 1,
            BaseCards::MERCH_3 => 2,
            BaseCards::MERCH_4 => 2,
            BaseCards::MERCH_5 => 3,
            BaseCards::MERCH_6 => 3,
            BaseCards::MERCH_7 => 4,
            BaseCards::MERCH_8 => 4,
            BaseCards::SPID_1 => 1,
            BaseCards::SPID_2 => 1,
            BaseCards::SPID_3 => 2,
            BaseCards::SPID_4 => 2,
            BaseCards::SPID_5 => 3,
            BaseCards::SPID_6 => 3,
            BaseCards::SPID_7 => 4,
            BaseCards::SPID_8 => 4,
            BaseCards::ROB_1 => 1,
            BaseCards::ROB_2 => 1,
            BaseCards::ROB_3 => 2,
            BaseCards::ROB_4 => 2,
            BaseCards::ROB_5 => 3,
            BaseCards::ROB_6 => 3,
            BaseCards::ROB_7 => 4,
            BaseCards::ROB_8 => 4,
        }
    }

    pub fn trigger(&self) -> Triggers {
        match self {
            BaseCards::MUSH_1 => Triggers::None,
            BaseCards::MUSH_2 => Triggers::Turn,
            BaseCards::MUSH_3 => Triggers::None,
            BaseCards::MUSH_4 => Triggers::Played,
            BaseCards::MUSH_5 => Triggers::Death,
            BaseCards::MUSH_6 => Triggers::Sold,
            BaseCards::MUSH_7 => Triggers::Turn,
            BaseCards::MUSH_8 => Triggers::Kill,
            BaseCards::MERCH_1 => Triggers::None,
            BaseCards::MERCH_2 => Triggers::Survived,
            BaseCards::MERCH_3 => Triggers::Death,
            BaseCards::MERCH_4 => Triggers::None,
            BaseCards::MERCH_5 => Triggers::Kill,
            BaseCards::MERCH_6 => Triggers::Turn,
            BaseCards::MERCH_7 => Triggers::Sold,
            BaseCards::MERCH_8 => Triggers::Survived,
            BaseCards::SPID_1 => Triggers::Played,
            BaseCards::SPID_2 => Triggers::None,
            BaseCards::SPID_3 => Triggers::Hit,
            BaseCards::SPID_4 => Triggers::None,
            BaseCards::SPID_5 => Triggers::Death,
            BaseCards::SPID_6 => Triggers::Hit,
            BaseCards::SPID_7 => Triggers::Turn,
            BaseCards::SPID_8 => Triggers::Turn,
            BaseCards::ROB_1 => Triggers::Played,
            BaseCards::ROB_2 => Triggers::None,
            BaseCards::ROB_3 => Triggers::Played,
            BaseCards::ROB_4 => Triggers::Turn,
            BaseCards::ROB_5 => Triggers::Hit,
            BaseCards::ROB_6 => Triggers::None,
            BaseCards::ROB_7 => Triggers::Turn,
            BaseCards::ROB_8 => Triggers::Turn,
        }
    }

    pub fn handle(&self, handles: &Res<TextureAssets>) -> Handle<ColorMaterial> {
        match self {
            BaseCards::MUSH_1 => handles.mush_1.clone(),
            BaseCards::MUSH_2 => handles.mush_2.clone(),
            BaseCards::MUSH_3 => handles.mush_3.clone(),
            BaseCards::MUSH_4 => handles.mush_4.clone(),
            BaseCards::MUSH_5 => handles.mush_5.clone(),
            BaseCards::MUSH_6 => handles.mush_6.clone(),
            BaseCards::MUSH_7 => handles.mush_7.clone(),
            BaseCards::MUSH_8 => handles.mush_8.clone(),
            BaseCards::MERCH_1 => handles.merch_1.clone(),
            BaseCards::MERCH_2 => handles.merch_2.clone(),
            BaseCards::MERCH_3 => handles.merch_3.clone(),
            BaseCards::MERCH_4 => handles.merch_4.clone(),
            BaseCards::MERCH_5 => handles.merch_5.clone(),
            BaseCards::MERCH_6 => handles.merch_6.clone(),
            BaseCards::MERCH_7 => handles.merch_7.clone(),
            BaseCards::MERCH_8 => handles.merch_8.clone(),
            BaseCards::SPID_1 => handles.spid_1.clone(),
            BaseCards::SPID_2 => handles.spid_2.clone(),
            BaseCards::SPID_3 => handles.spid_3.clone(),
            BaseCards::SPID_4 => handles.spid_4.clone(),
            BaseCards::SPID_5 => handles.spid_5.clone(),
            BaseCards::SPID_6 => handles.spid_6.clone(),
            BaseCards::SPID_7 => handles.spid_7.clone(),
            BaseCards::SPID_8 => handles.spid_8.clone(),
            BaseCards::ROB_1 => handles.rob_1.clone(),
            BaseCards::ROB_2 => handles.rob_2.clone(),
            BaseCards::ROB_3 => handles.rob_3.clone(),
            BaseCards::ROB_4 => handles.rob_4.clone(),
            BaseCards::ROB_5 => handles.rob_5.clone(),
            BaseCards::ROB_6 => handles.rob_6.clone(),
            BaseCards::ROB_7 => handles.rob_7.clone(),
            BaseCards::ROB_8 => handles.rob_8.clone(),
        }
    }
}

impl Card {
    pub(crate) fn new(card_type: BaseCards, id: u32) -> Self {
        match card_type {
            BaseCards::MUSH_1 => Card { id, base_card: card_type, atk: 1, hp: 3, ..Default::default() },
            BaseCards::MUSH_2 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::MUSH_3 => Card { id, base_card: card_type, atk: 2, hp: 5, ..Default::default() },
            BaseCards::MUSH_4 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::MUSH_5 => Card { id, base_card: card_type, atk: 0, hp: 2, ..Default::default() },
            BaseCards::MUSH_6 => Card { id, base_card: card_type, atk: 4, hp: 2, ..Default::default() },
            BaseCards::MUSH_7 => Card { id, base_card: card_type, atk: 6, hp: 2, ..Default::default() },
            BaseCards::MUSH_8 => Card { id, base_card: card_type, atk: 5, hp: 6, ..Default::default() },
            BaseCards::MERCH_1 => Card { id, base_card: card_type, atk: 1, hp: 3, ..Default::default() },
            BaseCards::MERCH_2 => Card { id, base_card: card_type, atk: 1, hp: 4, ..Default::default() },
            BaseCards::MERCH_3 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::MERCH_4 => Card { id, base_card: card_type, atk: 2, hp: 5, ..Default::default() },
            BaseCards::MERCH_5 => Card { id, base_card: card_type, atk: 5, hp: 3, ..Default::default() },
            BaseCards::MERCH_6 => Card { id, base_card: card_type, atk: 1, hp: 7, ..Default::default() },
            BaseCards::MERCH_7 => Card { id, base_card: card_type, atk: 4, hp: 8, ..Default::default() },
            BaseCards::MERCH_8 => Card { id, base_card: card_type, atk: 5, hp: 9, ..Default::default() },
            BaseCards::SPID_1 => Card { id, base_card: card_type, atk: 2, hp: 1, ..Default::default() },
            BaseCards::SPID_2 => Card { id, base_card: card_type, atk: 2, hp: 2, ..Default::default() },
            BaseCards::SPID_3 => Card { id, base_card: card_type, atk: 3, hp: 2, ..Default::default() },
            BaseCards::SPID_4 => Card { id, base_card: card_type, atk: 3, hp: 3, ..Default::default() },
            BaseCards::SPID_5 => Card { id, base_card: card_type, atk: 0, hp: 1, ..Default::default() },
            BaseCards::SPID_6 => Card { id, base_card: card_type, atk: 5, hp: 2, ..Default::default() },
            BaseCards::SPID_7 => Card { id, base_card: card_type, atk: 4, hp: 6, ..Default::default() },
            BaseCards::SPID_8 => Card { id, base_card: card_type, atk: 5, hp: 6, ..Default::default() },
            BaseCards::ROB_1 => Card { id, base_card: card_type, atk: 1, hp: 1, ..Default::default() },
            BaseCards::ROB_2 => Card { id, base_card: card_type, atk: 1, hp: 3, ..Default::default() },
            BaseCards::ROB_3 => Card { id, base_card: card_type, atk: 3, hp: 1, ..Default::default() },
            BaseCards::ROB_4 => Card { id, base_card: card_type, atk: 2, hp: 2, ..Default::default() },
            BaseCards::ROB_5 => Card { id, base_card: card_type, atk: 4, hp: 2, ..Default::default() },
            BaseCards::ROB_6 => Card { id, base_card: card_type, atk: 4, hp: 8, ..Default::default() },
            BaseCards::ROB_7 => Card { id, base_card: card_type, atk: 5, hp: 12, ..Default::default() },
            BaseCards::ROB_8 => Card { id, base_card: card_type, atk: 3, hp: 3, ..Default::default() },
        }
    }
}

pub(crate) struct CardPlugin;

pub struct NewCard(pub Entity, pub Card);

pub struct StatsChanged(pub Entity);

struct Prepare;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<NewCard>()
            .add_event::<StatsChanged>()
            .add_system_set(
                SystemSet::on_update(AppState::Shop)
                    .with_system(init_popup.system().label("popup:init"))
                    .with_system(update_size.system().label("popup:update").after("popup:init"))
                    .with_system(update_popup_visibility.system().after("popup:update"))
                    .with_system(update_stats.system())
            )
            .add_system_set(
                SystemSet::on_update(AppState::Fight)
                    .with_system(init_popup.system().label("popup:init"))
                    .with_system(update_size.system().label("popup:update").after("popup:init"))
                    .with_system(update_popup_visibility.system().after("popup:update"))
                    .with_system(update_stats.system())
            );
    }
}

struct Popup;

struct PopupBackground;

struct AtkStat;

struct HpStat;

const POPUP_X_OFFSET: f32 = 20.;
const POPUP_PADDING: f32 = 10.;

fn init_popup(
    mut commands: Commands,
    handles: Res<TextureAssets>,
    colors: Res<ColorAssets>,
    mut ev_new_card: EventReader<NewCard>,
    text_styles: Res<TextStyles>,
) {
    for new_card in ev_new_card.iter() {
        let base_card = new_card.1.base_card;
        commands.entity(new_card.0).with_children(|parent| {
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: format!("{}\n", base_card.name().to_string()),
                                style: text_styles.love_bug_small.clone(),
                            },
                            TextSection {
                                value: format!("Level {}\n\n", base_card.rank()),
                                style: text_styles.bird_seed_small.clone(),
                            },
                            TextSection {
                                value: if base_card.ability() != Abilities::None {
                                    format!("Ability: {}\nTriggered on {}\n\n", base_card.ability().to_string(), base_card.trigger().to_string())
                                } else { "".to_string() },
                                style: text_styles.bird_seed_small.clone(),
                            },
                            TextSection {
                                value: format!("{}\n\n", base_card.description().to_string()),
                                style: text_styles.bird_seed_small.clone(),
                            },
                        ],
                        ..Default::default()
                    },
                    visible: Visible {
                        is_visible: false,
                        is_transparent: true,
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, Z_POPUP_TEXT),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Popup)
                .insert(Prepare);

            parent
                .spawn_bundle(SpriteBundle {
                    material: colors.background.clone(),
                    sprite: Sprite::new(Vec2::new(0.0, 0.0)),
                    visible: Visible {
                        is_visible: false,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0., 0., Z_POPUP_BG),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PopupBackground)
                .insert(Prepare);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(format!("{}", new_card.1.hp),
                                             text_styles.stats.clone(),
                                             TextAlignment {
                                                 horizontal: HorizontalAlign::Center,
                                                 ..Default::default()
                                             }),
                    transform: Transform {
                        translation: Vec3::new(CARD_WIDTH / 4. / CARD_SCALE + 10., -CARD_HEIGHT / 2. / CARD_SCALE + 23., Z_STATS),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(HpStat);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(format!("{}", new_card.1.atk),
                                             text_styles.stats.clone(),
                                             TextAlignment {
                                                 horizontal: HorizontalAlign::Center,
                                                 ..Default::default()
                                             }),
                    transform: Transform {
                        translation: Vec3::new(-CARD_WIDTH / 4. / CARD_SCALE + 32., -CARD_HEIGHT / 2. / CARD_SCALE + 23., Z_STATS),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(AtkStat);
        });
    }
}

fn update_popup_visibility(
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(Entity, &Transform), (With<Dragged>, With<Card>)>,
        Query<(Entity, &Transform, &Children), With<Card>>,
    )>,
    mut visible_queries: QuerySet<(
        Query<(&mut Visible), With<Popup>>,
        Query<(&mut Visible), With<PopupBackground>>,
    )>,
) {
    // Get cursor position
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        // If a card is dragged and hovered it has popup priority
        let mut priority: Option<Entity> = None;
        for (e, transform) in queries.q1().iter() {
            let card_pos = transform.translation;
            if overlap(cursor.xyz(), card_pos, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)) {
                priority = Some(e);
                break;
            }
        }

        // Set visibilities
        for (e, transform, children) in queries.q2().iter() {
            let visible = match priority {
                Some(dragged_e) => e == dragged_e,
                None => overlap(cursor.xyz(), transform.translation, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)),
            };
            for child in children.iter() {
                if let Ok(mut visible_text) = visible_queries.q0_mut().get_mut(*child) {
                    visible_text.is_visible = visible;
                    continue;
                }
                if let Ok(mut visible_bg) = visible_queries.q1_mut().get_mut(*child) {
                    visible_bg.is_visible = visible;
                }
            }
        }
    }
}

fn update_size(
    mut commands: Commands,
    mut queries: QuerySet<(
        Query<(Entity, &Parent, &mut Transform, &Text2dSize), (With<Popup>, With<Prepare>)>,
        Query<(Entity, &Parent, &mut Sprite, &mut Transform), (With<PopupBackground>, With<Prepare>)>,
    )>,
) {
    let mut text_sizes: HashMap<Entity, Size> = HashMap::new();
    for (text_e, parent, mut transform, text_size) in queries.q0_mut().iter_mut() {
        let size = text_size.size;
        if size.width == 0. && size.height == 0. { break; }
        transform.translation.x = (CARD_WIDTH / 2. + size.width + POPUP_PADDING) / CARD_SCALE;
        transform.translation.y = (CARD_HEIGHT / 2. - size.height - POPUP_PADDING) / CARD_SCALE;
        commands.entity(text_e).remove::<Prepare>();
        text_sizes.insert(parent.0, size.clone());
    }

    for (bg_e, bg_parent, mut sprite, mut bg_transform) in queries.q1_mut().iter_mut() {
        if let Some(size) = text_sizes.get(&bg_parent.0) {
            sprite.size.x = (&size.width + 2. * POPUP_PADDING) / CARD_SCALE;
            sprite.size.y = (&size.height + 2. * POPUP_PADDING) / CARD_SCALE;
            bg_transform.translation.x = (CARD_WIDTH / 2. + &size.width / 2. + POPUP_PADDING) / CARD_SCALE;
            bg_transform.translation.y = (CARD_HEIGHT / 2. - &size.height / 2. - POPUP_PADDING) / CARD_SCALE;
            commands.entity(bg_e).remove::<Prepare>();
        }
    }
}

fn update_stats(
    mut ev_stats: EventReader<StatsChanged>,
    mut texts: QuerySet<(
        Query<(&Parent, &mut Text), With<AtkStat>>,
        Query<(&Parent, &mut Text), With<HpStat>>,
    )>,
    cards: Query<&Card>,
) {
    for event in ev_stats.iter() {
        if let Ok(card) = cards.get(event.0) {
            for (parent, mut text) in texts.q0_mut().iter_mut() {
                if parent.0 == event.0 {
                    text.sections[0].value = format!("{}", card.atk);
                    break;
                }
            }

            for (parent, mut text) in texts.q1_mut().iter_mut() {
                if parent.0 == event.0 {
                    text.sections[0].value = format!("{}", card.hp);
                    break;
                }
            }
        }
    }
}