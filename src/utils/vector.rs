use bevy::math::Vec3;

pub fn vec3_direction(base: Vec3, target: Vec3) -> Vec3 {
    let direction = target - base;
    direction.normalize()
}
