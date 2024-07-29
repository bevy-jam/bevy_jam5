use avian2d::prelude::{ColliderAabb, DebugRender};
use bevy::{color::palettes::tailwind, prelude::*};

use crate::dev_tools::editor::{HoveredObject, Polyline, SelectedObject};

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    polylines: Query<(&Polyline, &Transform, Entity)>,
    mut debug_render: Query<&mut DebugRender>,
    selected: Res<SelectedObject>,
    hovered: Res<HoveredObject>,
) {
    for (polyline, transform, entity) in polylines.iter() {
        let positions: Vec<_> = polyline
            .vertices
            .iter()
            .map(|n| (*transform * n.extend(0.0)).truncate())
            .collect();

        let selected = selected.0 == Some(entity);
        let hovered = hovered.0 == Some(entity);

        let color = if selected {
            tailwind::GREEN_200
        } else {
            tailwind::GRAY_300
        };

        for pos in positions.iter() {
            gizmos.circle_2d(*pos, 5.0, color);
        }

        let color = match (selected, hovered) {
            (true, _) => tailwind::AMBER_600,
            (_, true) => tailwind::AMBER_500,
            _ => tailwind::GRAY_300,
        };
        let color = match (selected, hovered) {
            (true, _) => tailwind::AMBER_600,
            (_, true) => tailwind::AMBER_500,
            _ => tailwind::GRAY_300,
        };

        gizmos.linestrip_2d(positions, color);

        if let Ok(mut dr) = debug_render.get_mut(entity) {
            let color = match (selected, hovered) {
                (true, _) => Some(tailwind::ORANGE_400.with_alpha(0.1)),
                (_, true) => Some(tailwind::ORANGE_200.with_alpha(0.2)),
                _ => None,
            };

            dr.aabb_color = color.map(Into::into);
        }
    }
}
