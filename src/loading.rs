use bevy::app::{AppBuilder, Plugin};
use bevy::ecs::system::Res;
use bevy::prelude::*;
use bevy::sprite::ColorMaterial;
use bevy_asset_loader::{AssetCollection};
use bevy_kira_audio::AudioSource;
use crate::AppState;

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(color_material)]
    #[asset(path = "MUSH_1.png")]
    pub mush_1: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MUSH_2.png")]
    pub mush_2: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MUSH_3.png")]
    pub mush_3: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MUSH_4.png")]
    pub mush_4: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MUSH_5.png")]
    pub mush_5: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MUSH_6.png")]
    pub mush_6: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MUSH_7.png")]
    pub mush_7: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MUSH_8.png")]
    pub mush_8: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_1.png")]
    pub merch_1: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_2.png")]
    pub merch_2: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_3.png")]
    pub merch_3: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_4.png")]
    pub merch_4: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_5.png")]
    pub merch_5: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_6.png")]
    pub merch_6: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_7.png")]
    pub merch_7: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_8.png")]
    pub merch_8: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_1.png")]
    pub spid_1: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_2.png")]
    pub spid_2: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_3.png")]
    pub spid_3: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_4.png")]
    pub spid_4: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_5.png")]
    pub spid_5: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_6.png")]
    pub spid_6: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_7.png")]
    pub spid_7: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_8.png")]
    pub spid_8: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_1.png")]
    pub rob_1: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_2.png")]
    pub rob_2: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_3.png")]
    pub rob_3: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_4.png")]
    pub rob_4: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_5.png")]
    pub rob_5: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_6.png")]
    pub rob_6: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_7.png")]
    pub rob_7: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_8.png")]
    pub rob_8: Handle<ColorMaterial>,

    #[asset(color_material)]
    #[asset(path = "shop.png")]
    pub shop_bg: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_TITLE.png")]
    pub title_bg: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "fight.png")]
    pub fight_bg: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "slot_border.png")]
    pub slot_border: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCHANT STORE.png")]
    pub shop_bob: Handle<ColorMaterial>,

    #[asset(color_material)]
    #[asset(path = "_ACTION_4.png")]
    pub refresh_button: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_ACTION_5.png")]
    pub freeze_button: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_ACTION_6.png")]
    pub upgrade_button: Handle<ColorMaterial>,

    #[asset(color_material)]
    #[asset(path = "ACTIONS BIS1.png")]
    pub heart: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ACTIONS BIS2.png")]
    pub exclamation: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_SABLIER1.png")]
    pub hourglass_1: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_SABLIER2.png")]
    pub hourglass_2: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_SABLIER3.png")]
    pub hourglass_3: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_SABLIER4.png")]
    pub hourglass_4: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "_SABLIER5.png")]
    pub hourglass_5: Handle<ColorMaterial>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    // BGM
    #[asset(path = "ost/1-title.ogg")]
    pub title: Handle<AudioSource>,
    #[asset(path = "ost/2-shop.ogg")]
    pub shop: Handle<AudioSource>,
    #[asset(path = "ost/3-combat.ogg")]
    pub fight: Handle<AudioSource>,
    #[asset(path = "ost/SFX/transition.ogg")]
    pub intro: Handle<AudioSource>,

    // SFX
    #[asset(path = "ost/SFX/buy.ogg")]
    pub buy_card: Handle<AudioSource>,
    #[asset(path = "ost/SFX/sell.ogg")]
    pub sell_card: Handle<AudioSource>,
    #[asset(path = "ost/SFX/refresh.ogg")]
    pub refresh: Handle<AudioSource>,
    #[asset(path = "ost/SFX/place_card.ogg")]
    pub place_card: Handle<AudioSource>,
    #[asset(path = "ost/SFX/freeze.ogg")]
    pub freeze: Handle<AudioSource>,
    #[asset(path = "ost/SFX/attack.ogg")]
    pub attack: Handle<AudioSource>,
    #[asset(path = "ost/SFX/death.ogg")]
    pub death: Handle<AudioSource>,
    #[asset(path = "ost/SFX/effect_1.ogg")]
    pub ability_triggered: Handle<AudioSource>,
    #[asset(path = "ost/SFX/level_up.ogg")]
    pub level_up: Handle<AudioSource>,
}

pub struct ColorAssets {
    pub background: Handle<ColorMaterial>,
    pub black: Handle<ColorMaterial>,
}
