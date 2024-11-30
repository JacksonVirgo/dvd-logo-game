use bevy::math::{Vec2, Vec3};

pub fn vec3_direction(base: Vec3, target: Vec3) -> Vec3 {
    let direction = target - base;
    direction.normalize()
}

pub fn vec2_direction(from: Vec2, to: Vec2) -> Vec2 {
    (to - from).normalize()
}
