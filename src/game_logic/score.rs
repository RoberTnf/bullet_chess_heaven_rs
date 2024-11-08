use bevy::prelude::*;

use crate::{
    pieces::{common::Team, health::PieceDeathEvent, player::experience::PieceValue},
    states::game_state::GameState,
};

#[derive(Resource)]
pub struct GameScore(pub usize);

impl FromWorld for GameScore {
    fn from_world(_world: &mut World) -> Self {
        GameScore(0)
    }
}

fn score_system(
    mut death_events: EventReader<PieceDeathEvent>,
    mut score: ResMut<GameScore>,
    value_query: Query<(&PieceValue, &Team)>,
) {
    for event in death_events.read() {
        let (value, team) = value_query.get(event.entity).unwrap();
        if *team != Team::Enemy {
            continue;
        }
        debug!("Adding to score: {}", value.value);
        score.0 += value.value;
    }
}

fn reset_score_system(mut score: ResMut<GameScore>) {
    debug!("Resetting score");
    score.0 = 0;
}

pub struct GameScorePlugin;

impl Plugin for GameScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .add_systems(Update, score_system)
            .add_systems(OnEnter(GameState::Game), reset_score_system);
    }
}
