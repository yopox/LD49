use std::fmt::{Display, Formatter};

use derive_more::{Add, Display, From, Into, Sub};
use rand::{Rng, thread_rng};

use crate::card::{Abilities, Card, Triggers};
use crate::PlayerData;

/* Notations:
    - att: attacking player
    - def: attacked player
    - hb: half-board of a player
    - id: uid of a card / player
    - index: index of a card inside a player's hand
 */

#[derive(Debug, Clone)]
pub enum CombatEvents {
    Attack { att_id: u16, att_card_index: u8, def_card_index: u8, def_card_hp_after: u16 },
    Death { player_id: u16, card_index: u8 },
    StatsChange { player_id: u16, card_index: u8, hp: i32, at: i32 },
    ApplyEffect { card_index: u8, player_id: u16 },
    GoldChange { player_id: u16, change: i32 },
}

impl Display for CombatEvents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatEvents::GoldChange { player_id, change } => write!(f, "GoldChange to={} of={} gold", player_id, change),
            CombatEvents::Attack { att_id, att_card_index: att_card_id, def_card_index: def_card_id, def_card_hp_after } => { write!(f, "Attack of {}.{} on {}.{} --> hp after = {}", att_id, att_card_id, 1 - att_id, def_card_id, def_card_hp_after) }
            CombatEvents::Death { player_id, card_index: card_id } => { write!(f, "Death of {}.{}", player_id, card_id) }
            CombatEvents::StatsChange { .. } => { write!(f, "Stats Change") }
            CombatEvents::ApplyEffect { .. } => { write!(f, "Effect") }
        }
    }
}

fn get_number_of_cards(b: &PlayerData) -> u8 {
    b.board.len() as u8
}

fn apply_effect(card_index: u8, opponent_card_index: u8, player_hb: &mut PlayerData, opponent_hb: &mut PlayerData, trigger: Triggers) -> Vec<CombatEvents> {
    let player_card = player_hb.board[card_index as usize];
    let opponent_card = opponent_hb.board[opponent_card_index as usize];
    let card_id = player_card.card_id;
    let player_id = player_hb.id;
    let opponent_id = opponent_hb.id;
    let mut events = vec![
        CombatEvents::ApplyEffect { card_index, player_id }
    ];

    match player_card.card_id.ability() {
        Abilities::ToxicSpores => events.push(CombatEvents::Death { player_id: opponent_hb.id, card_index: opponent_card_index }),
        Abilities::Gigantism => events.push(CombatEvents::StatsChange { player_id, card_index, hp: 0, at: 1 }),
        Abilities::Sadism => todo!("Not yet implemented"),
        Abilities::ExplodingArmour => {
            for mut card in &mut opponent_hb.board {
                if card.hp > 1 {
                    events.push(CombatEvents::StatsChange { player_id: opponent_id, hp: -1, at: 0, card_index });
                } else {
                    events.push(CombatEvents::Death { player_id: opponent_id, card_index });
                }
                card.hp = card.hp - 1;
            }
            opponent_hb.board.retain(|card| { card.hp > 0 });
        }
        Abilities::Pillage => events.push(CombatEvents::GoldChange { change: 1, player_id }),
        Abilities::Dexterity => todo!("Not yet implemented"),
        Abilities::Trap => events.push(CombatEvents::StatsChange { player_id: opponent_id, hp: 0, at: opponent_card.at as i32 / 2, card_index: opponent_card_index }),
        Abilities::Multiplication => todo!("Not yet implemented"),
        Abilities::Poisonous => events.push(CombatEvents::Death { player_id: opponent_id, card_index: opponent_card_index }),
        _ => {}
    };

    return events;
}

#[inline]
fn relu(i: i32) -> u16 {
    if i < 0 { 0 } else { i as u16 }
}

fn simulate_attack<T: Rng>(att_card_index: u8, att_hb: &mut PlayerData, def_hb: &mut PlayerData, rng: &mut T) -> Vec<CombatEvents> {
    let def_card_index = rng.gen_range(0..get_number_of_cards(def_hb)) as u8;
    let mut events = Vec::with_capacity(2);

    let att_card = &att_hb.board[att_card_index as usize];
    let att_card_trigger = att_card.card_id.trigger();
    let def_card = &def_hb.board[def_card_index as usize];
    let def_card_trigger = def_card.card_id.trigger();

    let def_post_hp = def_card.hp as i32 - att_card.at as i32;
    events.push(CombatEvents::Attack { att_card_index, att_id: att_hb.id, def_card_index, def_card_hp_after: relu(def_post_hp) });

    if def_post_hp <= 0 {
        // Dies
        events.push(CombatEvents::Death { player_id: def_hb.id, card_index: def_card_index });

        // Triggers Kill  or Hit on att
        if att_card_trigger == Triggers::Kill || att_card_trigger == Triggers::Hit {
            events.append(&mut apply_effect(att_card_index, def_card_index, att_hb, def_hb, att_card_trigger))
        }
        // Triggers Death or Hit on def
        if def_card_trigger == Triggers::Death || def_card_trigger == Triggers::Hit {
            events.append(&mut apply_effect(def_card_index, att_card_index, def_hb, att_hb, def_card_trigger))
        }

        def_hb.board.remove(def_card_index as usize);

        return events;
    }

    // Update card hp
    def_hb.board[def_card_index as usize].hp = def_post_hp as u16;
    // Update variables for use after move
    let att_card = &att_hb.board[att_card_index as usize];
    let def_card = &def_hb.board[def_card_index as usize];

    // Counter-attack
    let att_post_hp = att_card.hp as i32 - def_card.at as i32;
    events.push(CombatEvents::Attack { att_id: def_hb.id, def_card_index: att_card_index, att_card_index: def_card_index, def_card_hp_after: relu(att_post_hp) });

    if att_post_hp <= 0 {
        // Dies
        events.push(CombatEvents::Death { player_id: att_hb.id, card_index: att_card_index });

        // Triggers Kill or Hit on def
        if def_card_trigger == Triggers::Kill || def_card_trigger == Triggers::Hit {
            events.append(&mut apply_effect(def_card_index, att_card_index, def_hb, att_hb, def_card_trigger))
        }
        // Triggers Death or Hit on att
        if att_card_trigger == Triggers::Death || att_card_trigger == Triggers::Hit {
            events.append(&mut apply_effect(att_card_index, def_card_index, att_hb, def_hb, att_card_trigger))
        }

        att_hb.board.remove(att_card_index as usize);
    } else {
        att_hb.board[att_card_index as usize].hp = att_post_hp as u16;

        // Triggers Survived or Hit on att
        if att_card_trigger == Triggers::Survived || att_card_trigger == Triggers::Hit {
            events.append(&mut apply_effect(att_card_index, def_card_index, att_hb, def_hb, att_card_trigger))
        }
        // Triggers Hit on def
        if def_card_trigger == Triggers::Hit {
            events.append(&mut apply_effect(def_card_index, att_card_index, def_hb, att_hb, def_card_trigger));
        }
    }

    return events;
}

pub(crate) fn simulate_combat<T: Rng>(mut hb1: PlayerData, mut hb2: PlayerData, rng: &mut T) -> Vec<CombatEvents> {
    let mut events = Vec::new();

    let mut to_play = rng.gen::<bool>();
    let mut next_cards_to_play: [u8; 2] = [0, 0];

    // While each player has something to play
    while !hb1.board.is_empty() && !hb2.board.is_empty() {
        let (player_hb, oponent_hb) = if to_play { (&mut hb1, &mut hb2) } else { (&mut hb2, &mut hb1) };

        events.extend(simulate_attack(next_cards_to_play[to_play as usize], player_hb, oponent_hb, rng));

        let nb_cards = get_number_of_cards(if to_play { &hb1 } else { &hb2 });
        let quotient = if nb_cards > 0 { nb_cards } else { 1 };
        next_cards_to_play[to_play as usize] = (next_cards_to_play[to_play as usize] + 1) % quotient;
        to_play = !to_play;
    }
    return events;
}
