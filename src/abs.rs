use derive_more::{Add, Display, From, Into, Sub};

pub mod engine {
    /* Notations:
        - att: attacking player
        - def: attacked player
        - hb: half-board of a player
        - id: uid of a card / player
        - index: index of a card inside a player's hand
     */
    use std::collections::LinkedList;

    use rand::Rng;

    #[derive(Debug, Clone, Copy)]
    pub enum BoardModification {
        HPChange { new_hp: u16, player_id: usize, card_id: u16 },
        CardDeath { player_id: usize, card_id: u16 },
        PermanentHPChange { hp_change: i32, player_id: usize, card_id: u16 },
        PermanentAtChange { at_change: i32, player_id: usize, card_id: u16 },
        GoldChange { gold_change: i32, player_id: usize },
        NewCard { player_id: usize, card_id: u16 },
    }

    #[derive(Debug, Clone)]
    pub enum CombatEvent {
        Attack { att_id: usize, att_card_id: u16, def_card_id: u16, def_card_hp_after: u16 },
        Death { player_id: usize, card_id: u16 },
        ApplyEffect { effect: Effect, effect_trigger: EffectTrigger, card_id: u16, player_id: usize, stats_changes: Vec<BoardModification> },
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Effect {
        A,
        B,
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

    fn apply_effect<T: Rng>(card_index: usize, player_hb: &mut HalfBoard, oponent_hb: &mut HalfBoard, rng: &mut T) -> LinkedList<CombatEvent> {
        return LinkedList::new();
    }

    #[inline]
    fn relu(i: i32) -> u16 {
        if i < 0 { 0 } else { i as u16 }
    }

    fn simulate_attack<T: Rng>(att_card_index: usize, att_hb: &mut HalfBoard, def_hb: &mut HalfBoard, rng: &mut T) -> LinkedList<CombatEvent> {
        let def_card_index = rng.gen_range(0..get_number_of_cards(def_hb));
        let mut events = LinkedList::new();

        let att_card = &att_hb.cards[att_card_index];
        let att_card_trigger = att_card.effect.0;
        let def_card = &def_hb.cards[def_card_index];
        let def_card_trigger = def_card.effect.0;

        let def_post_hp = def_card.hp as i32 - att_card.at as i32;
        events.push_back(CombatEvent::Attack { att_card_id: att_card.id, att_id: att_hb.id, def_card_id: def_card.id, def_card_hp_after: relu(def_post_hp) });

        if def_post_hp <= 0 {
            // Dies
            events.push_back(CombatEvent::Death { player_id: def_hb.id, card_id: def_card.id });

            // Triggers KILL  or HIT on att
            if att_card_trigger == EffectTrigger::KILL || att_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(att_card_index, att_hb, def_hb, rng))
            }
            // Triggers DEATH or HIT on def
            if def_card_trigger == EffectTrigger::DEATH || def_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(def_card_index, def_hb, att_hb, rng))
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
        events.push_back(CombatEvent::Attack { att_id: def_hb.id, def_card_id: att_card.id, att_card_id: def_card.id, def_card_hp_after: relu(att_post_hp) });

        if att_post_hp <= 0 {
            // Dies
            events.push_back(CombatEvent::Death { player_id: att_hb.id, card_id: att_card.id });

            // Triggers KILL or HIT on def
            if def_card_trigger == EffectTrigger::KILL || def_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(def_card_index, def_hb, att_hb, rng))
            }
            // Triggers DEATH or HIT on att
            if att_card_trigger == EffectTrigger::DEATH || att_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(att_card_index, att_hb, def_hb, rng))
            }

            att_hb.cards.remove(att_card_index);
        } else {
            att_hb.cards[att_card_index].hp = att_post_hp as u16;

            // Triggers SURVIVED or HIT on att
            if att_card_trigger == EffectTrigger::SURVIVED || att_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(att_card_index, att_hb, def_hb, rng))
            }
            // Triggers HIT on def
            if def_card_trigger == EffectTrigger::HIT {
                events.append(&mut apply_effect(def_card_index, def_hb, att_hb, rng))
            }
        }

        return events;
    }

    fn simulate_combat<T: Rng>(mut hb1: HalfBoard, mut hb2: HalfBoard, rng: &mut T) -> LinkedList<CombatEvent> {
        let mut events = LinkedList::new();

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
