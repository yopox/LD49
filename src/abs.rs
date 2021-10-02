use derive_more::{Add, Sub, Display, From, Into};

pub mod engine {
    use std::collections::LinkedList;

    use rand::Rng;

    pub enum CombatEvent {
        ChangeStats(Card)
    }

    pub enum Effect {
        SurvivedAttack(usize, usize, u16)
    }

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

    pub type PlayerNum = bool;

    pub struct Card {
        id: u16,
        name: &'static str,
        hp: u16,
        at: u16,

        effect: (EffectTrigger, Effect),
    }

    pub struct Hero {
        id: u16,
        name: &'static str,
        effect: (EffectTrigger, Effect),
    }

    pub type AliveCards = Vec<Card>;

    pub struct HalfBoard {
        hp: u16,
        hero: Hero,
        cards: AliveCards,
    }

    fn get_number_of_cards(b: &HalfBoard) -> usize {
        b.cards.len()
    }

    fn simulate_attack<T: Rng>(attacking_card_index: usize, player_hb: &mut HalfBoard, oponent_hb: &mut HalfBoard, rng: &mut T) -> LinkedList<CombatEvent> {
        let attacked_card_index = rng.gen_range(0..get_number_of_cards(oponent_hb));

        let attacking_card = &mut player_hb.cards[attacking_card_index];
        let attacked_card = &mut oponent_hb.cards[attacked_card_index];
        let resulting_hp = attacked_card.hp as i32 - attacking_card.at as i32;
        if resulting_hp > 0 {
            // DIES
            oponent_hb.cards.remove(attacked_card_index);
            // Triggers KILL
            // Triggers DEATH
        } else {
            attacked_card.hp = resulting_hp as u16;
            // Triggers SURVIVED
        }

        return LinkedList::new();
    }

    fn simulate_combat<T: Rng>(mut hb1: HalfBoard, mut hb2: HalfBoard, rng: &mut T) -> LinkedList<CombatEvent> {
        let mut events = LinkedList::new();

        let mut to_play = rng.gen::<bool>();
        let mut next_cards_to_play: [usize; 2] = [0, 0];

        // While each player has something to play
        while !hb1.cards.is_empty() && !hb2.cards.is_empty() {
            let (player_hb, mut oponent_hb) = if (to_play) {(&mut hb1, &mut hb2)} else {(&mut hb2, &mut hb1)};

            events.extend(simulate_attack(next_cards_to_play[to_play as usize], player_hb, oponent_hb, rng));

            next_cards_to_play[to_play as usize] = (next_cards_to_play[to_play as usize] + 1) % get_number_of_cards(if (to_play) {&hb1} else {&hb2});
            to_play = !to_play;
        }
        return events;
    }
}
