use std::cmp::{max, min};
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
    Attack { att_id: u16, att_card_index: u8, def_card_index: u8 },
    EndOfAttack { att_id: u16, att_card_index: u8, def_card_index: u8 },
    Death { player_id: u16, card_id: u32 },
    StatsChange { player_id: u16, card_id: u32, hp: i32, at: i32 },
    ApplyAbility { card_index: u8, player_id: u16, ability: Abilities, card_id: u32 },
    GoldChange { player_id: u16, change: i32 },
    PlayersAttack { att_id: u16, change_def_hp: i32 },
}

impl Display for CombatEvents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatEvents::GoldChange { player_id, change } => write!(f, "GoldChange to={} of={} gold", player_id, change),
            CombatEvents::Attack { att_id, att_card_index: att_card_id, def_card_index: def_card_id } => { write!(f, "Attack of {}.{} on {}.{}", att_id, att_card_id, 1 - att_id, def_card_id) }
            CombatEvents::Death { player_id, card_id } => { write!(f, "Death of {}.{}", player_id, card_id) }
            CombatEvents::StatsChange { .. } => { write!(f, "Stats Change") }
            CombatEvents::ApplyAbility { card_index, player_id, ability, card_id } => { write!(f, "Effect {} of card {}.{}", ability, player_id, card_index) }
            CombatEvents::PlayersAttack { att_id, change_def_hp } => { write!(f, "Player {} takes {} to their opponent", att_id, change_def_hp) }
            CombatEvents::EndOfAttack { .. } => { write!(f, "End of attack") }
        }
    }
}

fn get_number_of_cards(b: &PlayerData) -> u8 {
    b.board.len() as u8
}

fn apply_effect<T: Rng>(card_index: u8, opponent_card_index: u8, player_hb: &mut PlayerData, opponent_hb: &mut PlayerData, rng: &mut T) -> Vec<CombatEvents> {
    let player_card = player_hb.board[card_index as usize];
    let opponent_card = opponent_hb.board[opponent_card_index as usize];
    let player_id = player_hb.id;
    let opponent_id = opponent_hb.id;
    let ability = player_card.base_card.ability();
    let mut events = vec![
        CombatEvents::ApplyAbility { card_index, player_id, ability, card_id: player_card.id }
    ];

    match ability {
        Abilities::ToxicSpores => {
            opponent_hb.board[opponent_card_index as usize].hp = 0;
        }
        Abilities::Gigantism => {
            player_hb.board[card_index as usize].atk += 1;
            events.push(CombatEvents::StatsChange { player_id, card_id: player_card.id, hp: 0, at: 1 })
        }
        Abilities::Sadism => {}, // TODO Not Yet Implemented
        Abilities::ExplodingArmour => {
            for mut card in &mut opponent_hb.board {
                events.push(CombatEvents::StatsChange { player_id: opponent_id, hp: -1, at: 0, card_id: card.id });
                card.hp = relu(card.hp as i32 - 1);
            }
        }
        Abilities::Pillage => events.push(CombatEvents::GoldChange { change: 1, player_id }),
        Abilities::Trap => {
            let change_atk = opponent_card.atk as i32 / 2;
            opponent_hb.board[opponent_card_index as usize].atk = change_atk as u16;
            events.push(CombatEvents::StatsChange { player_id: opponent_id, hp: 0, at: -change_atk, card_id: opponent_card.id })
        }
        Abilities::Multiplication => {}, // TODO: Not yet implemented
        Abilities::Poisonous => {
            opponent_hb.board[opponent_card_index as usize].hp = 0;
        }
        Abilities::Glitch => {
            if rng.gen() {
                opponent_hb.board[opponent_card_index as usize].atk = relu(opponent_card.atk as i32 - 2);
                events.push(CombatEvents::StatsChange { player_id: opponent_id, card_id: opponent_card.id, hp: 0, at: -2 });
            } else {
                opponent_hb.board[opponent_card_index as usize].hp = relu(opponent_card.hp as i32 - 2);
                events.push(CombatEvents::StatsChange { player_id: opponent_id, at: 0, hp: -2, card_id: opponent_card.id });
            }
        }
        _ => {}
    };

    return events;
}

#[inline]
fn relu(i: i32) -> u16 {
    if i < 0 { 0 } else { i as u16 }
}

#[inline]
fn min2<T: PartialOrd>(x: T, y: T) -> T { if x < y { x } else { y } }

fn simulate_attack<T: Rng>(att_card_index: usize, att_hb: &mut PlayerData, def_hb: &mut PlayerData, rng: &mut T) -> (Vec<CombatEvents>, bool) {
    let def_card_index = rng.gen_range(0..get_number_of_cards(def_hb));
    println!("Simulating attack of {}.({}/{}) on {}.({}/{})", att_hb.id, att_card_index, att_hb.board.len(), def_hb.id, def_card_index, def_hb.board.len());
    let mut events = Vec::with_capacity(2);

    let att_card = att_hb.board[att_card_index];
    let def_card = def_hb.board[def_card_index as usize];

    let att_card_index = att_card_index as u8;

    events.push(CombatEvents::Attack { att_card_index, att_id: att_hb.id, def_card_index });

    def_hb.board[def_card_index as usize].hp = relu(def_card.hp as i32 - att_card.atk as i32);
    events.push(CombatEvents::StatsChange { player_id: def_hb.id, card_id: def_card.id, at: 0, hp: -(att_card.atk as i32) });
    att_hb.board[att_card_index as usize].hp = relu(att_card.hp as i32 - def_card.atk as i32);
    events.push(CombatEvents::StatsChange { player_id: att_hb.id, card_id: att_card.id, at: 0, hp: -(def_card.atk as i32) });

    // Triggers
    let att_card_trigger = att_card.base_card.trigger();
    let def_card_trigger = def_card.base_card.trigger();
    if att_card_trigger == Triggers::Hit
        || (att_card_trigger == Triggers::Kill && def_card.hp <= 0)
        || (att_card_trigger == Triggers::Survived && att_card.hp > 0)
        || (att_card_trigger == Triggers::Death && att_card.hp <= 0)
    {
        events.append(&mut apply_effect(att_card_index, def_card_index, att_hb, def_hb, rng));
    }
    if def_card_trigger == Triggers::Hit
        || (def_card_trigger == Triggers::Kill && att_card.hp <= 0)
        || (def_card_trigger == Triggers::Death && def_card.hp <= 0)
    {
        events.append(&mut apply_effect(def_card_index, att_card_index, def_hb, att_hb, rng));
    }

    for &card in &att_hb.board {
        if card.hp <= 0 {
            events.push(CombatEvents::Death { player_id: att_hb.id, card_id: card.id });
        }
    };
    att_hb.board.retain(|card| card.hp > 0);
    for &card in &def_hb.board {
        if card.hp <= 0 {
            events.push(CombatEvents::Death { player_id: def_hb.id, card_id: card.id });
        }
    };
    def_hb.board.retain(|card| card.hp > 0);

    let mut survived = false;
    let mut att_card_index = att_card_index;
    for (i, &card) in att_hb.board.iter().enumerate() {
        if card.id == att_card.id {
            survived = true;
            att_card_index = i as u8;
        }
    }

    if survived {
        events.push(CombatEvents::EndOfAttack {att_card_index, att_id: att_hb.id, def_card_index})
    }

    let replay = survived && att_card_trigger == Triggers::Survived && att_card.base_card.ability() == Abilities::Dexterity
        && def_hb.board.len() > 0;

    return (events, replay);
}

pub(crate) fn simulate_combat<T: Rng>(mut hb1: PlayerData, mut hb2: PlayerData, rng: &mut T) -> Vec<CombatEvents> {
    let mut events = Vec::new();

    let mut to_play = rng.gen::<bool>();

    // While each player has something to play
    while !hb1.board.is_empty() && !hb2.board.is_empty() {
        let (player_hb, opponent_hb) = if to_play { (&mut hb1, &mut hb2) } else { (&mut hb2, &mut hb1) };

        let next_card_to_play = player_hb.board.iter().enumerate().map(|(i, card)| (card.played, i)).min().unwrap().1;
        player_hb.board[next_card_to_play].played += 1;

        let (new_events, replay) = simulate_attack(next_card_to_play, player_hb, opponent_hb, rng);
        events.extend(new_events);

        if replay {
            let (new_events, _) = simulate_attack(next_card_to_play, player_hb, opponent_hb, rng);
            events.extend(new_events);
        }

        to_play = !to_play;
    }

    let (winner, loser) = if hb1.board.is_empty() { (hb2, hb1) } else { (hb1, hb2) };
    let change_def_hp = -min2(loser.hp as i32, winner.board.iter().map(|card| card.base_card.rank() as i32).sum());
    events.push(CombatEvents::PlayersAttack {
        att_id: winner.id,
        change_def_hp,
    });

    return events;
}
