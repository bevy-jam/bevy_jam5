use avian2d::{math::AdjustPrecision, prelude::*};
use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingCircle},
    prelude::*,
};

#[derive(Component, Reflect, Debug, Clone)]
pub struct Polyline {
    pub vertices: Vec<Vec2>,
    pub indices: Vec<[u32; 2]>,
}

impl Polyline {
    pub fn new(pos: Vec2) -> Self {
        Self {
            vertices: vec![pos],
            indices: vec![],
        }
    }
}

impl Bounded2d for Polyline {
    fn aabb_2d(&self, translation: Vec2, rotation: impl Into<Rot2>) -> Aabb2d {
        Aabb2d::from_point_cloud(translation, rotation, &self.vertices)
    }

    fn bounding_circle(&self, translation: Vec2, rotation: impl Into<Rot2>) -> BoundingCircle {
        BoundingCircle::from_point_cloud(translation, rotation, &self.vertices)
    }
}

impl IntoCollider<Collider> for Polyline {
    fn collider(&self) -> Collider {
        let vertices = self.vertices.iter().map(|v| v.adjust_precision()).collect();
        Collider::polyline(vertices, self.indices.clone().into())
    }
}
