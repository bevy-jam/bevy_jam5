//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::Screen;
use crate::{
    game::assets::{ImageAssets, SfxAssets, SoundtrackAssets},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);

    let state = LoadingState::new(Screen::Loading)
        .continue_to_state(Screen::Title)
        .load_collection::<ImageAssets>()
        .load_collection::<SfxAssets>()
        .load_collection::<SoundtrackAssets>();

    let state = state.continue_to_state(Screen::Playing);

    app.add_loading_state(state);
}

fn enter_loading(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}
