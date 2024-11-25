use super::player::{Player, PLAYER_RADIUS, PLAYER_SPEED};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn player_movement(mut query: Query<(&mut Transform, &Player)>, time: Res<Time>) {
    for (mut transform, player) in &mut query {
        let dir = Vec3::new(player.direction.x, player.direction.y, 0.0);
        transform.translation += dir * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn player_collide_boundary(
    mut query: Query<(&mut Transform, &mut Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let x_min = 0.0 + PLAYER_RADIUS;
    let x_max = window.width() - PLAYER_RADIUS;
    let y_min = 0.0 + PLAYER_RADIUS;
    let y_max = window.height() - PLAYER_RADIUS;

    for (mut transform, mut player) in &mut query {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
            player.direction.x *= -1.0;
        } else if translation.x > x_max {
            translation.x = x_max;
            player.direction.x *= -1.0;
        }

        if translation.y < y_min {
            translation.y = y_min;
            player.direction.y *= -1.0;
        } else if translation.y > y_max {
            translation.y = y_max;
            player.direction.y *= -1.0;
        }

        transform.translation = translation;
    }
}
