use bevy::prelude::*;

use crate::{
    game::{assets::SfxAssets, audio::sfx::PlaySfx},
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (apply_interaction_palette, trigger_interaction_sfx)
            .run_if(not(in_state(Screen::Splash)))
            .run_if(not(in_state(Screen::Loading))),
    );
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;

/// Palette for widget interactions.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: InteractionQuery<(&InteractionPalette, &mut BackgroundColor)>,
) {
    for (interaction, (palette, mut background)) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn trigger_interaction_sfx(
    mut interactions: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
    sfx_asssets: Res<SfxAssets>,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => {
                commands.trigger(PlaySfx::Handle(sfx_asssets.button_hover.clone_weak()))
            }
            Interaction::Pressed => {
                commands.trigger(PlaySfx::Handle(sfx_asssets.button_press.clone_weak()))
            }
            _ => (),
        }
    }
}
