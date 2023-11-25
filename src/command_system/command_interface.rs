use bevy::prelude::*;

use crate::app_state::state::AppState;

use super::events::CommandDispatchEvent;

#[derive(Component)]
pub struct CommandInterface {}

pub fn spawn_command_interface(mut commands: Commands, command_history: ResMut<CommandHistory>) {
    let _command_interface_entity = build_command_interface(&mut commands, command_history);
}

pub fn despawn_command_interface(
    mut commands: Commands,
    command_interface_query: Query<Entity, With<CommandInterface>>,
) {
    if let Ok(command_interface_entity) = command_interface_query.get_single() {
        commands
            .entity(command_interface_entity)
            .despawn_recursive();
    }
}
// Some super hacky stuff here, command history is preloaded with a "/" in order to do two things
// it makes the "/" key always prefix commands with /
// it also makes it so that the command interface is actually visible
// not sure how to do this otherwise.
// command history [0] is always "/" in this case
// we insert all subsequent commands to [1]. pushing the commands into a reverse order for easier navigation
#[allow(clippy::too_many_arguments)]
pub fn update_command_interface(
    mut command_history: ResMut<CommandHistory>,
    mut command_history_index: ResMut<CommandHistoryIndex>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut event_reader_char: EventReader<ReceivedCharacter>,
    mut command_dispatch_event_writer: EventWriter<CommandDispatchEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut command_interface_query: Query<&mut Text, With<CommandInterface>>,
) {
    if string.is_empty() {
        for char in command_history.commands[0].chars() {
            string.push(char)
        }
    }
    if let Some(key) = keyboard_input.get_just_pressed().next() {
        let command_history_length = command_history.commands.len();
        match key {
            KeyCode::Up => {
                string.clear();
                command_history_index.index =
                    if command_history_index.index + 1 > command_history_length - 1 {
                        command_history_length - 1
                    } else {
                        command_history_index.index + 1
                    };
                for char in command_history.commands[command_history_index.index].chars() {
                    string.push(char)
                }
            }
            KeyCode::Down => {
                string.clear();
                command_history_index.index = if command_history_index.index as isize - 1_isize < 0
                {
                    0
                } else {
                    command_history_index.index - 1
                };
                for char in command_history.commands[command_history_index.index].chars() {
                    string.push(char)
                }
            }
            KeyCode::Return | KeyCode::NumpadEnter => {
                command_dispatch_event_writer.send(CommandDispatchEvent {
                    command: string.to_string(),
                });
                next_app_state.set(AppState::Game);
                command_history.commands.insert(1, string.to_string());
                command_history_index.index = 0;
                string.clear();
            }
            KeyCode::Back => {
                string.pop();
            }
            KeyCode::Escape => {
                string.clear();
            }
            _ => {}
        }
    }
    for ev in event_reader_char.read() {
        // ignore control (special) characters
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }
    for mut text in &mut command_interface_query {
        text.sections[0].value = string.to_string();
    }
}

pub fn build_command_interface(
    commands: &mut Commands,
    mut command_history: ResMut<CommandHistory>,
) -> Entity {
    if command_history.commands.is_empty() {
        command_history.commands.push("/".to_string());
    }
    let command_interface_entity = commands
        .spawn((
            TextBundle {
                visibility: Visibility::Visible,
                background_color: BackgroundColor(Color::GRAY),
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    ..default()
                },
                text: Text::from_section(
                    command_history.commands[0].clone(),
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..Default::default()
            },
            CommandInterface {},
        ))
        .id();
    command_interface_entity
}

#[derive(Resource)]
pub struct CommandHistory {
    pub commands: Vec<String>,
}
#[derive(Resource)]
pub struct CommandHistoryIndex {
    pub index: usize,
}
