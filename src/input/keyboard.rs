use bevy::prelude::*;

use crate::{
    globals::{REFRESH_SHOP_COST, REFRESH_SHOP_KEY, SHOP_KEY},
    ui::shop::RefreshShop,
};

#[derive(Event)]
pub struct ToggleShop;

pub fn toggle_shop(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<ToggleShop>,
) {
    if keyboard_input.just_pressed(SHOP_KEY) {
        event_writer.send(ToggleShop);
    }
}

pub fn refresh_shop(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<RefreshShop>,
) {
    if keyboard_input.just_pressed(REFRESH_SHOP_KEY) {
        event_writer.send(RefreshShop {
            cost: REFRESH_SHOP_COST,
        });
    }
}

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_shop);
        app.add_event::<ToggleShop>();
        app.add_systems(Update, refresh_shop);
        app.add_event::<RefreshShop>();
    }
}
