use bevy::ecs::system::Query;
use bevy::math::{vec2, Vec4Swizzles};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use crate::{GlobalData, MainCamera, MySelf, PlayerData};
use crate::data::card::{Card, CARD_HEIGHT, CARD_WIDTH};
use crate::data::loading::{AudioAssets, TextureAssets};
use crate::shop::shop_manager::ShopManager;
use crate::shop::shop_screen::{add_card, ButtonText, FreezeButton, Hourglass, RefreshButton, ShopSlot, ShopSlots, ShopValues, ShopFrozen, StartFight, UpgradeButton};
use crate::util::{cursor_pos, overlap};
use crate::ui::drag_and_drop::Draggable;
use crate::ui::card_overlay::NewCard;

pub(crate) fn handle_buttons(
    mut player_data: Query<&mut PlayerData, With<MySelf>>,
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    shop_values: Res<ShopValues>,
    main_camera: Query<&Transform, With<MainCamera>>,
    queries: QuerySet<(
        Query<&Transform, With<Hourglass>>,
        Query<&Transform, With<RefreshButton>>,
        Query<&Transform, With<FreezeButton>>,
        Query<&Transform, With<UpgradeButton>>,
    )>,
    card_query: Query<(Entity, &Card, &ShopSlot)>,
    mut button_text: Query<&mut Text, With<ButtonText>>,
    mut frozen_shop: ResMut<ShopFrozen>,
    mut commands: Commands,
    mut global_data: ResMut<GlobalData>,
    handles: Res<TextureAssets>,
    mut ev_new_card: EventWriter<NewCard>,
    mut ev_fight: EventWriter<StartFight>,
    audio: Res<Audio>,
    music: Res<AudioAssets>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, main_camera.single().unwrap()) {
        let mut player_data = player_data.single_mut().unwrap();

        let transform = queries.q1().single().unwrap();
        if overlap(cursor.xyz(), transform.translation, (50., 50.)) {
            button_text.single_mut().unwrap().sections[0].value = format!("Refresh cards for {} coins.", shop_values.refresh);
            if btn.just_pressed(MouseButton::Left) && player_data.coins >= shop_values.refresh {
                audio.play_in_channel(music.refresh.clone(), &AudioChannel::new("SFX".to_owned()));
                player_data.coins -= shop_values.refresh;
                *frozen_shop = ShopFrozen(None);
                for (e, _card, &slot) in card_query.iter() {
                    if slot.row == ShopSlots::SHOP {
                        commands.entity(e).despawn_recursive();
                    }
                }
                for (i, &base_card) in ShopManager::shop_inventory(player_data.shop_level, &mut global_data.rng).iter().enumerate() {
                    let id = global_data.next_card_id;
                    global_data.next_card_id += 1;
                    let card_id = add_card(Card::new(base_card, id),
                                           ShopSlot { row: ShopSlots::SHOP, id: i as u8 },
                                           &mut commands, &handles, &mut ev_new_card);
                    commands
                        .entity(card_id)
                        .insert(Draggable { size: vec2(CARD_WIDTH / 2., CARD_HEIGHT / 2.) });
                }
            }
            return;
        }

        let transform = queries.q2().single().unwrap();
        if overlap(cursor.xyz(), transform.translation, (50., 50.)) {
            button_text.single_mut().unwrap().sections[0].value =
                if frozen_shop.0.is_none() {
                    format!("Freeze cards for {} coins.", shop_values.freeze)
                } else {
                    "Shop already frozen.".to_string()
                };
            if btn.just_pressed(MouseButton::Left) && player_data.coins >= shop_values.freeze && frozen_shop.0.is_none() {
                audio.play_in_channel(music.freeze.clone(), &AudioChannel::new("SFX".to_owned()));
                player_data.coins -= shop_values.freeze;
                frozen_shop.0 = Some(
                    card_query.iter()
                        .filter_map(|(_, &card, &slot)|
                            if slot.row == ShopSlots::SHOP {
                                Some((slot.id, card))
                            } else { None })
                        .collect()
                );
            };
            return;
        }

        let transform = queries.q3().single().unwrap();
        if overlap(cursor.xyz(), transform.translation, (50., 50.)) {
            let upgrade_cost: i16 = match player_data.shop_level {
                1 => 4,
                2 => 6,
                3 => 8,
                _ => -1,
            };
            if upgrade_cost == -1 {
                button_text.single_mut().unwrap().sections[0].value = "The shop can't be upgraded anymore.".to_string();
                return;
            } else {
                button_text.single_mut().unwrap().sections[0].value = format!("Upgrade the shop for {} coins.", upgrade_cost);
                if btn.just_pressed(MouseButton::Left) && player_data.coins >= upgrade_cost as u16 {
                    audio.play_in_channel(music.level_up.clone(), &AudioChannel::new("SFX".to_owned()));
                    player_data.coins -= upgrade_cost as u16;
                    player_data.shop_level += 1;
                }
                return;
            }
        }

        let transform = queries.q0().single().unwrap();
        if overlap(cursor.xyz(), transform.translation, (60., 70.)) {
            button_text.single_mut().unwrap().sections[0].value = "Click to end your turn.".to_string();
            if btn.just_pressed(MouseButton::Left) {
                ev_fight.send(StartFight);
            }
            return;
        }
    }
    button_text.single_mut().unwrap().sections[0].value = "".to_string();
}