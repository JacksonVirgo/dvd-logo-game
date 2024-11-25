use bevy::{
    app::AppExit,
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "DVD Logo Game".to_string(),
                resolution: (1024.0, 576.0).into(),
                resizable: false,
                mode: WindowMode::Windowed,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit.send(AppExit::Success);
    }
}
