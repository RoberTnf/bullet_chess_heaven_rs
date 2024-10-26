use bevy::prelude::*;

use crate::globals::{DARKER_PRIMARY_COLOR, PRIMARY_COLOR, SECONDARY_COLOR};

#[derive(Event, Clone)]
pub struct ButtonPressedEvent {
    pub function: ButtonFunction,
}

#[derive(Component, Clone)]
pub enum ButtonFunction {
    Restart,
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonFunction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<ButtonPressedEvent>,
) {
    for (interaction, mut color, mut border_color, button_function) in &mut interaction_query {
        debug!("Button system running!");
        match *interaction {
            Interaction::Hovered => {
                *color = DARKER_PRIMARY_COLOR.into();
                border_color.0 = SECONDARY_COLOR;
            }
            Interaction::Pressed => {
                event_writer.send(ButtonPressedEvent {
                    function: button_function.clone(),
                });
            }
            _ => {
                *color = PRIMARY_COLOR.into();
                border_color.0 = SECONDARY_COLOR;
            }
        }
    }
}

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
        app.add_event::<ButtonPressedEvent>();
    }
}
