use derive_more::{Add, Display, From, Into, Sub};

pub mod engine {
    /* Notations:
        - att: attacking player
        - def: attacked player
        - hb: half-board of a player
        - id: uid of a card / player
        - index: index of a card inside a player's hand
     */
    use rand::Rng;

    use crate::abs::engine::CombatEvent::GoldChange;

    #[derive(Debug, Clone)]
    pub enum CombatEvent {
        Attack { att_id: usize, att_card_id: u16, def_card_id: u16, def_card_hp_after: u16 },
        Death { player_id: usize, card_id: u16 },
        StatsChange { player_id: usize, card_id: u16, hp: i32, at: i32 },
        ApplyEffect { effect: Effect, effect_trigger: EffectTrigger, card_id: u16, player_id: usize },
        GoldChange { player_id: usize, change: i32 },
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Effect {
        // Mushrooms
        ToxicSpores,
        Gigantism,
        // Merchants
        Sadism,
        ExplodingArmour,
        Pillage,
        Dexterity,
        // Spiders
        Trap,
        Multiplication,
        Poisonous,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum EffectTrigger {
        // At the beginning of each turn
        TURN,
        // When this card is played
        PLAYED,
        // When this card dies
        DEATH,
        // When this card attacks and survives
        SURVIVED,
        // When this card attacks or is attacked
        HIT,
        // When this card kills
        KILL,
        // When this card is sold
        SOLD,
        // PASSIVE,
    }

    #[derive(Copy, Clone)]
    pub struct Card {
        id: u16,
        name: &'static str,
        hp: u16,
        at: u16,

        effect: (EffectTrigger, Effect),
    }

    #[derive(Copy, Clone)]
    pub struct Hero {
        id: u16,
        name: &'static str,
        effect: (EffectTrigger, Effect),
    }

    pub type AliveCards = Vec<Card>;

    #[derive(Clone)]
    pub struct HalfBoard {
        id: usize,
        hp: u16,
        hero: Hero,
        cards: AliveCards,
    }

    fn get_number_of_cards(b: &HalfBoard) -> usize {
        b.cards.len()
    }

    fn apply_effect<T: Rng>(card_index: usize, opponent_card_index: usize, player_hb: &mut HalfBoard, opponent_hb: &mut HalfBoard, trigger: EffectTrigger, rng: &mut T) -> Vec<CombatEvent> {
        let player_card = player_hb.cards[card_index];
        let opponent_card = opponent_hb.cards[opponent_card_index];
        let card_id = player_card.id;
        let player_id = player_hb.id;
        let opponent_id = opponent_hb.id;
        let mut events = vec![
            CombatEvent::ApplyEffect { effect: player_card.effect.1, effect_trigger: trigger, card_id, player_id }
        ];

        match player_card.effect.1 {
            Effect::ToxicSpores => events.push(CombatEvent::Death { player_id: opponent_hb.id, card_id: opponent_card.id }),
            Effect::Gigantism => events.push(CombatEvent::StatsChange { player_id, card_id, hp: 0, at: 1 }),
            Effect::Sadism => todo!("Not yet implemented"),
            Effect::ExplodingArmour => {
                for mut card in &mut opponent_hb.cards {
                    if card.hp > 1 {
                        events.push(CombatEvent::StatsChange { player_id: opponent_id, hp: -1, at: 0, card_id: card.id });
                    } else {
                        events.push(CombatEvent::Death { player_id: opponent_id, card_id: card.id });
                    }
                    card.hp = card.hp - 1;
                }
                opponent_hb.cards.retain(|card| { card.hp > 0 });
            }
            Effect::Pillage => events.push(CombatEvent::GoldChange { change: 1, player_id }),
            Effect::Dexterity => todo!("Not yet implemented"),
            Effect::Trap => events.push(CombatEvent::StatsChange {player_id: opponent_id, hp: 0, at: opponent_card.at as i32 / 2, card_id: opponent_card.id}),
            Effect::Multiplication => todo!("Not yet implemented"),
            Effect::Poisonous => events.push(CombatEvent::Death {player_id: opponent_id, card_id: opponent_card.id}),
        };

        return events;
    }

    #[inline]
    fn relu(i: i32) -> u16 {
        if i < 0 { 0 } else { i as u16 }
    }

    fn simulate_attack<T: Rng>(att_card_index: usize, att_hb: &mut HalfBoard, def_hb: &mut HalfBoard, rng: &mut T) -> Vec<CombatEvent> {
        let def_card_index = rng.gen_range(0..get_number_of_cards(def_hb));
        let mut events = Vec::with_capacity(2);

        let att_card = &att_hb.cards[att_card_index];
        let att_card_trigger = att_card.effect.0;
        let def_card = &def_hb.cards[def_card_index];
        let def_card_trigger = def_card.effect.0;

        let def_post_hp = def_card.hp as i32 - att_card.at as i32;
        events.push(CombatEvent::Attack { att_card_id: att_card.id, att_id: att_hb.id, def_card_id: def_card.id, def_card_hp_after: relu(def_post_hp) });

        if def_post_hp <= 0 {
            // Dies
            events.push(CombatEvent::Death { player_id: def_hb.id, card_id: def_card.id });

            // Triggers KILL  or HIT on att
            if att_card_trigger == EffectTrigger::KILL || att_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(att_card_index, def_card_index, att_hb, def_hb, att_card_trigger, rng))
            }
            // Triggers DEATH or HIT on def
            if def_card_trigger == EffectTrigger::DEATH || def_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(def_card_index, att_card_index, def_hb, att_hb, def_card_trigger, rng))
            }

            def_hb.cards.remove(def_card_index);

            return events;
        }

        // Update card hp
        def_hb.cards[def_card_index].hp = def_post_hp as u16;
        // Update variables for use after move
        let att_card = &att_hb.cards[att_card_index];
        let def_card = &def_hb.cards[def_card_index];

        // Counter-attack
        let att_post_hp = att_card.hp as i32 - def_card.at as i32;
        events.push(CombatEvent::Attack { att_id: def_hb.id, def_card_id: att_card.id, att_card_id: def_card.id, def_card_hp_after: relu(att_post_hp) });

        if att_post_hp <= 0 {
            // Dies
            events.push(CombatEvent::Death { player_id: att_hb.id, card_id: att_card.id });

            // Triggers KILL or HIT on def
            if def_card_trigger == EffectTrigger::KILL || def_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(def_card_index, att_card_index, def_hb, att_hb, def_card_trigger, rng))
            }
            // Triggers DEATH or HIT on att
            if att_card_trigger == EffectTrigger::DEATH || att_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(att_card_index, def_card_index, att_hb, def_hb, att_card_trigger, rng))
            }

            att_hb.cards.remove(att_card_index);
        } else {
            att_hb.cards[att_card_index].hp = att_post_hp as u16;

            // Triggers SURVIVED or HIT on att
            if att_card_trigger == EffectTrigger::SURVIVED || att_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(att_card_index, def_card_index, att_hb, def_hb, att_card_trigger, rng))
            }
            // Triggers HIT on def
            if def_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(def_card_index, att_card_index, def_hb, att_hb, def_card_trigger, rng))
            }
        }

        return events;
    }

    fn simulate_combat<T: Rng>(mut hb1: HalfBoard, mut hb2: HalfBoard, rng: &mut T) -> Vec<CombatEvent> {
        let mut events = Vec::new();

        let mut to_play = rng.gen::<bool>();
        let mut next_cards_to_play: [usize; 2] = [0, 0];

        // While each player has something to play
        while !hb1.cards.is_empty() && !hb2.cards.is_empty() {
            let (player_hb, oponent_hb) = if to_play { (&mut hb1, &mut hb2) } else { (&mut hb2, &mut hb1) };

            events.extend(simulate_attack(next_cards_to_play[to_play as usize], player_hb, oponent_hb, rng));

            next_cards_to_play[to_play as usize] = (next_cards_to_play[to_play as usize] + 1) % get_number_of_cards(if to_play { &hb1 } else { &hb2 });
            to_play = !to_play;
        }
        return events;
    }
}
