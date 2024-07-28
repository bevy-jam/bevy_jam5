use bevy::input::ButtonInput;
use bevy::prelude::*;

#[derive(States, Debug, Default, Hash, Eq, PartialEq, Clone)]
pub enum EditorState {
    #[default]
    SelectMode,
    CreateMode,
    MoveMode,
    EditMode,
}

pub fn mode_switch_system(
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<EditorState>>,
) {
    let Some(kc) = input.get_just_pressed().next() else {
        return;
    };

    match kc {
        KeyCode::KeyM => state.set(EditorState::MoveMode),
        KeyCode::KeyN => state.set(EditorState::SelectMode),
        KeyCode::KeyB => state.set(EditorState::EditMode),
        _ => (),
    }
}
