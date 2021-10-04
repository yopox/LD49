use rand::distributions::{Distribution, Standard};
use rand::Rng;
use crate::shop::ShopValues;

pub enum ShopRules {
    BuyPlus,
    BuyMinus,
    SellPlus,
    SellMinus,
    RefreshPlus,
    RefreshMinus,
    RefreshInf,
    FreezePlus,
    FreezeInf,
    TimerPlus,
    TimerMinus,
    None,
}

impl Distribution<ShopRules> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShopRules {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=22) { // rand 0.8
            1|2 => ShopRules::BuyPlus,
            3 => ShopRules::BuyMinus,
            4 => ShopRules::SellPlus,
            5|6 => ShopRules::SellMinus,
            7|8 => ShopRules::RefreshPlus,
            9 => ShopRules::RefreshMinus,
            10|11 => ShopRules::RefreshInf,
            12|13 => ShopRules::FreezePlus,
            14|15 => ShopRules::FreezeInf,
            16 => ShopRules::TimerPlus,
            17|18 => ShopRules::TimerMinus,
            _ => ShopRules::None,
        }
    }
}

impl ShopRules {
    fn edit_values(&self, values: &mut ShopValues) {
        match self {
            ShopRules::BuyPlus => values.buy = 4,
            ShopRules::BuyMinus => values.buy = 2,
            ShopRules::SellPlus => values.sell = -2,
            ShopRules::SellMinus => values.sell = 0,
            ShopRules::RefreshPlus => values.refresh = 2,
            ShopRules::RefreshMinus => values.refresh = 0,
            ShopRules::RefreshInf => values.refresh = 99,
            ShopRules::FreezePlus => values.freeze = 1,
            ShopRules::FreezeInf => values.refresh = 99,
            ShopRules::TimerPlus => values.timer = 90.,
            ShopRules::TimerMinus => values.timer = 30.,
            ShopRules::None => {}
        }
    }

    fn description(&self) -> &'static str {
        match self {
            ShopRules::BuyPlus => "\"I have less cards lately...\nI had to increase prices!\"\n\n(Cards cost 4 coins this turn.)",
            ShopRules::BuyMinus => "\"We will receive new cards soon...\nThere is a sale on all cards!\"\n\n(Cards cost 2 coins this turn.)",
            ShopRules::SellPlus => "\"My son wants to finish\nhis collection!\nCome and sell your cards!\"\n\n(Selling cards gives 2 coins this turn.)",
            ShopRules::SellMinus => "\"The shop is running low on coins...\nI can't accept returns today.\"\n\n(Selling cards gives 0 coins this turn.)",
            ShopRules::RefreshPlus => "\"I don't have enough time\nto show you all my cards today...\nIt will be more expensive.\"\n\n(Refreshing cards costs 2 coins this turn.)",
            ShopRules::RefreshMinus => "\"So much cards have arrived!\nWill you take a look at them all?\"\n\n(Refreshing cards is free this turn.)",
            ShopRules::RefreshInf => "\"All the cards are sold!\nI spared a few for you!\"\n\n(Can't refresh cards this turn.)",
            ShopRules::FreezePlus => "\"My storage is nearly full!\nIf you want me to keep some cards\nfor next time this will not be free...\"\n(Freezing cards costs 1 coin this turn.)",
            ShopRules::FreezeInf => "\"My storage is full!\nI can't keep any cards today, sorry.\"\n\n(Can't freeze cards this turn.)",
            ShopRules::TimerPlus => "\"You are early today!\nTake your time :)\"\n\n(The timer lasts 90s this turn.)",
            ShopRules::TimerMinus => "\"I will close soon...\nPlease hurry.\"\n\n(The timer lasts 30s this turn.)",
            ShopRules::None => "\"Enjoy your time in the shop!\"\n\n(No effect this turn.)",
        }
    }

    pub fn random(values: &mut ShopValues) -> &'static str {
        let rule: ShopRules = rand::random();
        rule.edit_values(values);
        return rule.description();
    }
}