use avian2d::prelude::ColliderAabb;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::dev_tools::editor::{Polyline, SelectedObject};

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    polylines: Query<(&Polyline, &Transform, Entity)>,
    collider_aabbs: Query<&ColliderAabb>,
    selected: Res<SelectedObject>,
) {
    for (polyline, transform, entity) in polylines.iter() {
        let positions: Vec<_> = polyline
            .vertices
            .iter()
            .map(|n| (*transform * n.extend(0.0)).truncate())
            .collect();

        let highlight = selected.0 == Some(entity);

        let color = if highlight {
            tailwind::GREEN_200
        } else {
            tailwind::GRAY_300
        };

        for pos in positions.iter() {
            gizmos.circle_2d(*pos, 5.0, color);
        }

        let color = if highlight {
            tailwind::AMBER_200
        } else {
            tailwind::GRAY_300
        };

        gizmos.linestrip_2d(positions, color);
    }

    for (collider_aabb) in collider_aabbs.iter() {}
}
