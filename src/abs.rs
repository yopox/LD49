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

    #[derive(Copy, Clone)]
    pub enum CombatEvent {
        Attack { att_id: usize, att_card_index: usize, def_card_index: usize, def_card_hp_after: u16 },
        Death { player_id: usize, card_index: usize },
        ApplyEffect { effect: Effect, effect_trigger: EffectTrigger, card_index: usize, player_id: usize },
    }

    #[derive(Copy, Clone)]
    pub enum Effect {
        A,
        B,
    }

    #[derive(Copy, Clone)]
    pub enum EffectTrigger {
        TURN,
        // At the beginning of each turn
        PLAYED,
        // When this card is played
        DEATH,
        // When this card dies
        KILL,
        // When this card is sold
        SURVIVED,
        // When this card attacks and survives
        SOLD, // When this card is sold
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

    #[inline]
    fn relu(i: i32) -> u16 {
        if i < 0 { 0 } else { i as u16 }
    }

    // att --> attacking
    // def --> attacked
    fn simulate_attack<T: Rng>(att_card_index: usize, att_hb: &mut HalfBoard, def_hb: &mut HalfBoard, rng: &mut T) -> LinkedList<CombatEvent> {
        let def_card_index = rng.gen_range(0..get_number_of_cards(def_hb));
        let mut events = LinkedList::new();

        let att_card = &mut att_hb.cards[att_card_index];
        let def_card = &mut def_hb.cards[def_card_index];

        let def_post_hp = def_card.hp as i32 - att_card.at as i32;
        events.push_back(CombatEvent::Attack { att_card_index, att_id: att_hb.id, def_card_index, def_card_hp_after: relu(def_post_hp) });

        if def_post_hp <= 0 {
            // Dies
            def_hb.cards.remove(def_card_index);
            events.push_back(CombatEvent::Death { player_id: def_hb.id, card_index: def_card_index });

            // Triggers KILL on def
            // Triggers DEATH on def
            // Triggers HIT on both

            return events;
        }

        // Update card hp
        def_card.hp = def_post_hp as u16;

        // Counter-attack
        let att_post_hp = att_card.hp as i32 - def_card.at as i32;
        events.push_back(CombatEvent::Attack { att_id: def_hb.id, def_card_index: att_card_index, att_card_index: def_card_index, def_card_hp_after: relu(att_post_hp) });

        if att_post_hp <= 0 {
            // Dies
            att_hb.cards.remove(att_card_index);
            events.push_back(CombatEvent::Death { player_id: att_hb.id, card_index: att_card_index })

            // Triggers KILL on def
            // Triggers DEATH on att
            // Triggers HIT on both
        } else {
            // Triggers SURVIVED
        }

        return events;
    }

    fn simulate_combat<T: Rng>(mut hb1: HalfBoard, mut hb2: HalfBoard, rng: &mut T) -> LinkedList<CombatEvent> {
        let mut events = LinkedList::new();

        let mut to_play = rng.gen::<bool>();
        let mut next_cards_to_play: [usize; 2] = [0, 0];

        // While each player has something to play
        while !hb1.cards.is_empty() && !hb2.cards.is_empty() {
            let (player_hb, mut oponent_hb) = if (to_play) { (&mut hb1, &mut hb2) } else { (&mut hb2, &mut hb1) };

            events.extend(simulate_attack(next_cards_to_play[to_play as usize], player_hb, oponent_hb, rng));

            next_cards_to_play[to_play as usize] = (next_cards_to_play[to_play as usize] + 1) % get_number_of_cards(if (to_play) { &hb1 } else { &hb2 });
            to_play = !to_play;
        }
        return events;
    }
}
