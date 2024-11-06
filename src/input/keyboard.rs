use bevy::prelude::*;

use crate::globals::SHOP_KEY;

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

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_shop);
        app.add_event::<ToggleShop>();
    }
}
