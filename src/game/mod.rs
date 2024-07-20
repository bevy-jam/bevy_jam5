//! Game mechanics and content.

use bevy::prelude::*;
use crate::game::spawn::level::{ForestMap, ForestTile};
use crate::game::spawn::player::Player;
use crate::screen::Screen;

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
    ));
    app.add_systems(FixedUpdate, burn.run_if(in_state(Screen::Playing)));
}

pub const GROWTH_RATE : f32 = 0.001;

pub fn burn(mut query: Query<(Entity, &mut ForestTile, &mut Handle<ColorMaterial>, &Transform)>, mut materials: ResMut<Assets<ColorMaterial>>, map: Res<ForestMap>, player_query: Query<(&Transform), With<Player>>) {
    for (_, mut ft, mut cm, t) in query.iter_mut() {
                    let mut new_growth= (ft.growth+GROWTH_RATE).min(1.);
                    if let Ok(p) = player_query.get_single() {
                        let pos = t.translation.xy();
                        if (p.translation.xy()-pos).length() <= 4. {
                            new_growth = 0.
                        }
                    }
                    if new_growth != ft.growth {
                        if let Some(m) = materials.get_mut(&mut *cm) {
                            m.color = Color::srgba((1.-new_growth).max(0.), 1.,(new_growth-0.2).min(0.4),new_growth);
                        }
                    }
                    ft.growth = new_growth;
    }
}