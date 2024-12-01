use bevy::prelude::*;

use crate::{globals::UI_FONT_SIZE, ui::button::ButtonFunction};

use super::{button::ButtonPressedEvent, setup_ui, LeftUINode};

#[derive(Event, Debug, Default)]
pub struct MessageEvent {
    pub message: String,
    pub timer: Option<Timer>,
}

#[derive(Component)]
struct MessageContainer;

#[derive(Component)]
struct Message {
    timer: Option<Timer>,
    text: String,
}

fn spawn_message_container(mut commands: Commands, root_query: Query<Entity, With<LeftUINode>>) {
    let root = root_query.get_single().expect("Root not found");
    let id = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MessageContainer,
            GlobalZIndex(1),
        ))
        .id();
    commands.entity(root).add_child(id);
}

fn message_system(
    mut messages: EventReader<MessageEvent>,
    mut commands: Commands,
    container_query: Query<Entity, With<MessageContainer>>,
    current_messages_query: Query<&Message>,
) {
    for message in messages.read() {
        let container = container_query.get_single().expect("Container not found");
        debug!("Message: {:?}", message);
        let current_message_texts: Vec<String> = current_messages_query
            .iter()
            .map(|m| m.text.clone())
            .collect();
        if current_message_texts.contains(&message.message) {
            continue;
        }
        let id = commands
            .spawn((
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                BorderRadius::all(Val::Px(4.0)),
                BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
                Message {
                    timer: message.timer.clone(),
                    text: message.message.clone(),
                },
                Button,
                ButtonFunction::CloseMessage,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text(message.message.clone()),
                    TextFont {
                        // TODO: Choose a font for long texts
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                ));
            })
            .id();
        commands.entity(container).add_child(id);
    }
}

fn despawn_message_container(
    mut commands: Commands,
    mut message_query: Query<(&mut Message, Entity)>,
    time: Res<Time>,
) {
    for (mut message, entity) in message_query.iter_mut() {
        if let Some(timer) = &mut message.timer {
            if timer.tick(time.delta()).just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn close_message(
    mut commands: Commands,
    message_query: Query<Entity, With<Message>>,
    mut event_reader: EventReader<ButtonPressedEvent>,
) {
    for event in event_reader.read() {
        if event.function == ButtonFunction::CloseMessage {
            if let Ok(entity) = message_query.get(event.entity) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageEvent>();
        app.add_systems(Update, message_system.run_if(on_event::<MessageEvent>));
        app.add_systems(Startup, spawn_message_container.after(setup_ui));
        app.add_systems(Update, despawn_message_container);
        app.add_systems(Update, close_message.run_if(on_event::<ButtonPressedEvent>));
    }
}
