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
    #[asset(path = "MUSH_8.png")]
    pub mush_8: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCH_8.png")]
    pub merch_8: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "SPID_8.png")]
    pub spid_8: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "ROB_8.png")]
    pub rob_8: Handle<ColorMaterial>,

    #[asset(color_material)]
    #[asset(path = "shop.png")]
    pub shop_bg: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "fight.png")]
    pub fight_bg: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "slot_border.png")]
    pub slot_border: Handle<ColorMaterial>,
    #[asset(color_material)]
    #[asset(path = "MERCHANT STORE.png")]
    pub shop_bob: Handle<ColorMaterial>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "ost/1- title.ogg")]
    pub title: Handle<AudioSource>,
    #[asset(path = "ost/3- shop.ogg")]
    pub shop: Handle<AudioSource>,
    #[asset(path = "ost/2- combat.ogg")]
    pub fight: Handle<AudioSource>,
}

pub struct ColorAssets {
    pub background: Handle<ColorMaterial>,
}
