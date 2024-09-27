use bevy::prelude::*;
pub mod click_tile;
mod update_pos;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, click_tile::tile_clicked)
            .add_systems(Update, update_pos::update_position)
            .add_event::<click_tile::TileClickedEvent>()
            .add_event::<update_pos::UpdatePositionEvent>();
    }
}
