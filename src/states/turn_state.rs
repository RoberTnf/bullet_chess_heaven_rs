use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    Player,
    Environment,
    Enemy,
}
