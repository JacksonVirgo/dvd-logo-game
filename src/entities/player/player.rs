use crate::utils::vector::vec2_direction;

use super::movement;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use rand::random;

pub const PLAYER_RADIUS: f32 = 16.0;
pub const PLAYER_SPEED: f32 = 200.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                movement::player_movement,
                movement::player_collide_boundary,
                aim_reticle,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct Player {
    pub direction: Vec2,
    pub name: String,
}

#[derive(Component)]
pub struct AimingRetical {
    pub name: String,
}

#[derive(Bundle)]
pub struct AimingReticalBundle {
    pub aiming_retical: AimingRetical,
    pub sprite: MaterialMesh2dBundle<ColorMaterial>,
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
    let color = Color::srgba(255.0, 255.0, 255.0, 0.25);
    let aiming_col = Color::srgba(255.0, 0.0, 0.0, 1.0);

    let dir_x = random::<f32>();
    let dir_y = random::<f32>();
    let dir = Vec2::new(dir_x, dir_y).normalize();

    let player_entity = commands
        .spawn((
            Player {
                direction: dir,
                name: "PLAYEr".to_string(),
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(PLAYER_RADIUS))),
                material: materials.add(color),
                transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            AimingRetical {
                name: "YAY".to_string(),
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(PLAYER_RADIUS, PLAYER_RADIUS))),
                material: materials.add(aiming_col),
                transform: Transform::from_xyz(32.0, 0.0, 1.0),
                ..default()
            },
        ))
        .set_parent(player_entity);
}

fn aim_reticle(
    player_query: Query<(&Transform, &Player)>,
    mut retical_query: Query<(&mut Transform, &mut AimingRetical), Without<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window_query.single();
    let cursor_position = window.cursor_position();

    if let Some(cursor_position) = cursor_position {
        let (camera, camera_transform) = camera_query.single();
        if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
            let cursor = world_position.get_point(0.0);
            for (parent_transform, _) in player_query.iter() {
                for (mut ret_transform, _) in retical_query.iter_mut() {
                    let direction =
                        vec2_direction(parent_transform.translation.truncate(), cursor.truncate());
                    ret_transform.translation = Vec3::new(
                        direction.x * 32.0,
                        direction.y * 32.0,
                        ret_transform.translation.z,
                    );
                }
            }
        }
    }
}
