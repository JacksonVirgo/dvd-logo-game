use bevy::{
    prelude::*,
    render::view::window,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform,
    window::PrimaryWindow,
};

use rand::random;

pub const PLAYER_RADIUS: f32 = 16.0;
pub const PLAYER_SPEED: f32 = 200.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
        app.add_systems(Update, player_collide_boundary);
    }
}

#[derive(Component)]
pub struct Player {
    pub direction: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single();

    let pos_x = window.width() / 2.0 - PLAYER_RADIUS;
    let pos_y = window.height() / 2.0 - PLAYER_RADIUS;
    let color = Color::srgb(255.0, 255.0, 255.0);

    let dir_x = random::<f32>();
    let dir_y = random::<f32>();
    let dir = Vec2::new(dir_x, dir_y).normalize();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: PLAYER_RADIUS,
            })),
            material: materials.add(color),
            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
            ..default()
        },
        Player { direction: dir },
    ));
}

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
