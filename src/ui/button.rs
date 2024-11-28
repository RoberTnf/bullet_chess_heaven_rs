use bevy::prelude::*;

use crate::{
    globals::{
        PRIMARY_COLOR_GRAYED, PRIMARY_COLOR_GRAYED_BRIGHTER, REFRESH_SHOP_COST, SECONDARY_COLOR,
    },
    states::game_state::GameState,
};

use super::shop::RefreshShop;

#[derive(Event, Clone)]
pub struct ButtonPressedEvent {
    pub function: ButtonFunction,
    pub entity: Entity,
}

#[derive(Event, Clone)]
pub struct ButtonHoverEvent {
    pub entity: Entity,
    pub function: ButtonFunction,
}

#[derive(Component, Clone, Eq, PartialEq)]
pub enum ButtonFunction {
    RestartGame,
    BuyUpgrade,
    RefreshShop,
    ShowShop,
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonFunction,
            Entity,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<ButtonPressedEvent>,
    mut hover_event_writer: EventWriter<ButtonHoverEvent>,
) {
    for (interaction, mut color, mut border_color, button_function, entity) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Hovered => {
                *color = PRIMARY_COLOR_GRAYED_BRIGHTER.into();
                border_color.0 = SECONDARY_COLOR;
                hover_event_writer.send(ButtonHoverEvent {
                    function: button_function.clone(),
                    entity,
                });
            }
            Interaction::Pressed => {
                event_writer.send(ButtonPressedEvent {
                    function: button_function.clone(),
                    entity,
                });
            }
            _ => {
                *color = PRIMARY_COLOR_GRAYED.into();
                border_color.0 = SECONDARY_COLOR;
            }
        }
    }
}

pub fn handle_button_pressed(
    mut event_reader: EventReader<ButtonPressedEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut refresh_shop_event_writer: EventWriter<RefreshShop>,
) {
    for event in event_reader.read() {
        match event.function {
            ButtonFunction::RestartGame => {
                game_state.set(GameState::Restart);
            }
            ButtonFunction::RefreshShop => {
                refresh_shop_event_writer.send(RefreshShop {
                    cost: REFRESH_SHOP_COST,
                });
            }
            _ => {}
        }
    }
}

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
        app.add_systems(Update, handle_button_pressed);
        app.add_event::<ButtonPressedEvent>();
        app.add_event::<ButtonHoverEvent>();
    }
}
