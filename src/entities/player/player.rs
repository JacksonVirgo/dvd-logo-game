use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

use super::movement;
use rand::random;

pub const PLAYER_RADIUS: f32 = 16.0;
pub const PLAYER_SPEED: f32 = 200.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (movement::player_movement, movement::player_collide_boundary).chain(),
        );
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
