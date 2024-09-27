use bevy::prelude::*;

use crate::board;
use crate::pieces;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, pieces::player::spawn_player)
            .add_systems(Startup, board::tile::spawn_board);
    }
}
