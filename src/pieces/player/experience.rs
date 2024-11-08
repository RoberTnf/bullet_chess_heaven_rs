use bevy::prelude::*;

use crate::{
    pieces::{common::Team, health::PieceDeathEvent},
    states::game_state::GameState,
};

#[derive(Component)]
pub struct PieceValue {
    pub value: usize,
}

#[derive(Event)]
pub struct PlayerLevelUpEvent {
    pub level: usize,
}

#[derive(Resource)]
pub struct PlayerLevel {
    pub level: usize,
    pub experience: usize,
}

impl FromWorld for PlayerLevel {
    fn from_world(_world: &mut World) -> Self {
        PlayerLevel::new()
    }
}

impl PlayerLevel {
    pub fn new() -> Self {
        PlayerLevel {
            level: 1,
            experience: 0,
        }
    }

    pub fn add_experience(&mut self, amount: usize) {
        debug!("Adding experience: {}", amount);
        self.experience = self.experience.saturating_add(amount);
    }

    pub fn level_up(&mut self) -> bool {
        if self.experience >= self.get_exp_to_next_level() {
            self.experience = self.experience.saturating_sub(self.get_exp_to_next_level());
            self.level += 1;
            debug!("Leveled up to {}", self.level);
            true
        } else {
            false
        }
    }

    pub fn get_exp_to_next_level(&self) -> usize {
        self.level * 2
    }
}

pub fn add_experience_on_death(
    mut piece_death_events: EventReader<PieceDeathEvent>,
    piece_value_query: Query<(&PieceValue, &Team)>,
    mut player_level: ResMut<PlayerLevel>,
    mut player_level_up_events: EventWriter<PlayerLevelUpEvent>,
) {
    for event in piece_death_events.read() {
        let (piece_value, team) = piece_value_query.get(event.entity).unwrap();
        if *team != Team::Enemy {
            return;
        }
        player_level.add_experience(piece_value.value);
        if player_level.level_up() {
            player_level_up_events.send(PlayerLevelUpEvent {
                level: player_level.level,
            });
        }
    }
}

pub fn reset_player_level(mut player_level: ResMut<PlayerLevel>) {
    debug!("Resetting player level");
    *player_level = PlayerLevel::new();
}

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLevel>();
        app.add_event::<PlayerLevelUpEvent>();
        app.add_systems(OnEnter(GameState::Game), reset_player_level);
        app.add_systems(
            Update,
            add_experience_on_death.run_if(in_state(GameState::Game)),
        );
    }
}
